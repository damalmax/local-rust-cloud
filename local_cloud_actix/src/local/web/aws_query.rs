use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::future::Future;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use actix_http::header::CONTENT_LENGTH;
use actix_http::{HttpMessage, Payload};
use actix_web::dev::Decompress;
use actix_web::web::BytesMut;
use actix_web::{web, Error, FromRequest, HttpRequest};
use encoding_rs::{Encoding, UTF_8};
use futures_core::future::LocalBoxFuture;
use futures_core::ready;
use futures_util::{FutureExt, StreamExt};
use serde::de::DeserializeOwned;

use crate::local::web::aws_query_error::AwsQueryEncodedError;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AwsQuery<T>(pub T);

impl<T> AwsQuery<T> {
    /// Unwraps into inner `T` value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for AwsQuery<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for AwsQuery<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> FromRequest for AwsQuery<T>
where
    T: DeserializeOwned + 'static,
{
    type Error = Error;
    type Future = AwsQueryExtractFut<T>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let AwsQueryConfig { limit, err_handler } = AwsQueryConfig::from_req(req).clone();

        AwsQueryExtractFut {
            fut: AwsQueryEncoded::new(req, payload).limit(limit),
            req: req.clone(),
            err_handler,
        }
    }
}

type AwsQueryErrHandler = Option<Rc<dyn Fn(AwsQueryEncodedError, &HttpRequest) -> actix_web::error::Error>>;

pub struct AwsQueryExtractFut<T> {
    fut: AwsQueryEncoded<T>,
    err_handler: AwsQueryErrHandler,
    req: HttpRequest,
}

impl<T> Future for AwsQueryExtractFut<T>
where
    T: DeserializeOwned + 'static,
{
    type Output = Result<AwsQuery<T>, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        let res = ready!(Pin::new(&mut this.fut).poll(cx));

        let res = match res {
            Err(err) => match &this.err_handler {
                Some(err_handler) => Err((err_handler)(err, &this.req)),
                None => Err(err.into()),
            },
            Ok(item) => Ok(AwsQuery(item)),
        };

        Poll::Ready(res)
    }
}

#[derive(Clone)]
struct AwsQueryConfig {
    limit: usize,
    err_handler: AwsQueryErrHandler,
}

impl AwsQueryConfig {
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    pub fn error_handler<F>(mut self, f: F) -> Self
    where
        F: Fn(AwsQueryEncodedError, &HttpRequest) -> actix_web::error::Error + 'static,
    {
        self.err_handler = Some(Rc::new(f));
        self
    }

    fn from_req(req: &HttpRequest) -> &Self {
        req.app_data::<Self>()
            .or_else(|| req.app_data::<web::Data<Self>>().map(|d| d.as_ref()))
            .unwrap_or(&DEFAULT_CONFIG)
    }
}

impl<T: Display> Display for AwsQuery<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

const DEFAULT_CONFIG: AwsQueryConfig = AwsQueryConfig {
    limit: 16_384, // (~16kb)
    err_handler: None,
};

impl Default for AwsQueryConfig {
    fn default() -> Self {
        DEFAULT_CONFIG
    }
}

pub struct AwsQueryEncoded<T> {
    stream: Option<Decompress<Payload>>,
    limit: usize,
    length: Option<usize>,
    encoding: &'static Encoding,
    err: Option<AwsQueryEncodedError>,
    fut: Option<LocalBoxFuture<'static, Result<T, AwsQueryEncodedError>>>,
}

// #[allow(clippy::borrow_interior_mutable_const)]
impl<T> AwsQueryEncoded<T> {
    pub fn new(req: &HttpRequest, payload: &mut Payload) -> Self {
        // check content type
        if req.content_type().to_lowercase() != "application/x-www-form-urlencoded" {
            return Self::err(AwsQueryEncodedError::ContentType);
        }
        let encoding = match req.encoding() {
            Ok(enc) => enc,
            Err(_) => return Self::err(AwsQueryEncodedError::ContentType),
        };

        let mut len = None;
        if let Some(l) = req.headers().get(&CONTENT_LENGTH) {
            if let Ok(s) = l.to_str() {
                if let Ok(l) = s.parse::<usize>() {
                    len = Some(l)
                } else {
                    return Self::err(AwsQueryEncodedError::UnknownLength);
                }
            } else {
                return Self::err(AwsQueryEncodedError::UnknownLength);
            }
        };

        let payload = Decompress::from_headers(payload.take(), req.headers());

        AwsQueryEncoded {
            encoding,
            stream: Some(payload),
            limit: 32_768,
            length: len,
            fut: None,
            err: None,
        }
    }

    fn err(err: AwsQueryEncodedError) -> Self {
        AwsQueryEncoded {
            stream: None,
            limit: 32_768,
            fut: None,
            err: Some(err),
            length: None,
            encoding: UTF_8,
        }
    }

    /// Set maximum accepted payload size. The default limit is 256kB.
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }
}

impl<T> Future for AwsQueryEncoded<T>
where
    T: DeserializeOwned + 'static,
{
    type Output = Result<T, AwsQueryEncodedError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(ref mut fut) = self.fut {
            return Pin::new(fut).poll(cx);
        }

        if let Some(err) = self.err.take() {
            return Poll::Ready(Err(err));
        }

        // payload size
        let limit = self.limit;
        if let Some(len) = self.length.take() {
            if len > limit {
                return Poll::Ready(Err(AwsQueryEncodedError::Overflow { size: len, limit }));
            }
        }

        // future
        let encoding = self.encoding;
        let mut stream = self.stream.take().unwrap();

        self.fut = Some(
            async move {
                let mut body = BytesMut::with_capacity(8192);

                while let Some(item) = stream.next().await {
                    let chunk = item?;

                    if (body.len() + chunk.len()) > limit {
                        return Err(AwsQueryEncodedError::Overflow {
                            size: body.len() + chunk.len(),
                            limit,
                        });
                    } else {
                        body.extend_from_slice(&chunk);
                    }
                }

                // if encoding == UTF_8 {
                //     serde_aws_query_ce::from_bytes::<T>(&body).map_err(AwsQueryEncodedError::Parse)
                // } else {
                let body = encoding
                    .decode_without_bom_handling_and_without_replacement(&body)
                    .map(Cow::into_owned)
                    .ok_or(AwsQueryEncodedError::Encoding)?;

                serde_aws_query_ce::from_str::<T>(&body).map_err(AwsQueryEncodedError::Parse)
                // }
            }
            .boxed_local(),
        );

        self.poll(cx)
    }
}
