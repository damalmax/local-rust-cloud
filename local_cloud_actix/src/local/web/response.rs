use std::convert::Infallible;
use std::mem;
use std::ops::Deref;
use std::pin::Pin;
use std::task::{Context, Poll};

use actix_http::body::{BodySize, MessageBody};
use actix_web::web::Bytes;

pub struct XmlResponse(pub String);

impl Default for XmlResponse {
    fn default() -> Self {
        XmlResponse(String::new())
    }
}

impl Deref for XmlResponse {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl MessageBody for XmlResponse {
    type Error = Infallible;

    #[inline]
    fn size(&self) -> BodySize {
        self.0.size()
    }

    #[inline]
    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Result<Bytes, Self::Error>>> {
        if self.0.is_empty() {
            Poll::Ready(None)
        } else {
            let string = mem::take(self.get_mut()).0;
            Poll::Ready(Some(Ok(Bytes::from(string))))
        }
    }

    #[inline]
    fn try_into_bytes(self) -> Result<Bytes, Self>
    where
        Self: Sized,
    {
        Ok(Bytes::from(self.0))
    }
}
