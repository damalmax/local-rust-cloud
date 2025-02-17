use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

use axum::body::Bytes;
use axum::extract::{FromRequest, Request};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::de::DeserializeOwned;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AwsQueryBody<T>(pub T);

impl<T> AwsQueryBody<T> {
    /// Unwraps into inner `T` value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> AwsQueryBody<T>
where
    T: DeserializeOwned,
{
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, AwsQueryRejection> {
        let entity: T = local_aws_query_protocol::from_bytes(bytes.as_ref())
            .map_err(|_err| AwsQueryRejection::FailedToDeserializeAwsQueryBody)?;
        Ok(AwsQueryBody(entity))
    }
}

impl<T> Deref for AwsQueryBody<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for AwsQueryBody<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, S> FromRequest<S> for AwsQueryBody<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = AwsQueryRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let bytes = Bytes::from_request(req, state)
            .await
            .map_err(|_err| AwsQueryRejection::Bytes)?;

        AwsQueryBody::from_bytes(bytes.as_ref())
    }
}

#[derive(Debug)]
pub enum AwsQueryRejection {
    FailedToDeserializeAwsQueryBody,
    Bytes,
}

impl IntoResponse for AwsQueryRejection {
    fn into_response(self) -> Response {
        let body = format!("{}", self);

        match self {
            AwsQueryRejection::FailedToDeserializeAwsQueryBody => (StatusCode::BAD_REQUEST, body).into_response(),
            AwsQueryRejection::Bytes => (StatusCode::INTERNAL_SERVER_ERROR, body).into_response(),
        }
    }
}

impl Display for AwsQueryRejection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let body = match self {
            AwsQueryRejection::FailedToDeserializeAwsQueryBody => {
                "AwsQuery: failed to deserialize parameters from request body."
            }
            AwsQueryRejection::Bytes => {
                "AwsQuery: failed to deserialize parameters from query string. Failed to read bytes from request body."
            }
        };
        write!(f, "{}", body)
    }
}

impl std::error::Error for AwsQueryRejection {}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::Request;
    use axum::routing::post;
    use axum::Router;
    use http_body_util::BodyExt;
    use serde::Deserialize;
    use tower::ServiceExt;
    use tower_http::trace::TraceLayer;

    use crate::local::aws_query::AwsQueryBody;

    #[tokio::test]
    async fn parse_aws_query_from_body() {
        #[derive(Debug, Deserialize)]
        struct Input {
            foo: String,
        }

        let app = Router::new()
            .route("/", post(|payload: AwsQueryBody<Input>| async move { payload.0.foo }))
            // We can still add middleware
            .layer(TraceLayer::new_for_http());

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/")
                    .body(Body::from("foo=bar"))
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();

        assert_eq!(&body[..], b"bar");
    }
}
