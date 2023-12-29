use actix_http::error::PayloadError;
use actix_http::StatusCode;
use actix_web::ResponseError;
use derive_more::{Display, Error, From};

use serde_aws_query_ce::DeError;

#[derive(Debug, Display, Error, From)]
pub enum AwsQueryEncodedError {
    #[display(
        fmt = "AWS Query encoded payload is larger ({} bytes) than allowed (limit: {} bytes)",
        size,
        limit
    )]
    Overflow { size: usize, limit: usize },

    #[display(fmt = "Payload size is now known.")]
    UnknownLength,

    #[display(fmt = "Content type error.")]
    ContentType,

    #[display(fmt = "Parse error: {}.", _0)]
    Parse(DeError),

    #[display(fmt = "Encoding error.")]
    Encoding,

    #[display(fmt = "Error that occur during reading payload: {}.", _0)]
    Payload(PayloadError),
}

impl ResponseError for AwsQueryEncodedError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Overflow { .. } => StatusCode::PAYLOAD_TOO_LARGE,
            Self::UnknownLength => StatusCode::LENGTH_REQUIRED,
            Self::Payload(err) => err.status_code(),
            _ => StatusCode::BAD_REQUEST,
        }
    }
}
