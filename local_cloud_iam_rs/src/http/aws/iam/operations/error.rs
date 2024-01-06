use std::error::Error;
use std::fmt::{Display, Formatter};

use local_cloud_validate::ValidationError;

use crate::http::aws::iam::actions::error::ApiErrorKind;

#[derive(Debug)]
pub(crate) enum OperationError {
    Service { kind: ApiErrorKind, msg: String },
    Validation(ValidationError),
}

impl Display for OperationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationError::Service { kind, msg } => {
                writeln!(f, "Operation Error. Kind: {}, Error Message: {}", kind, msg)
            }
            OperationError::Validation(error) => error.fmt(f),
        }
    }
}

impl Error for OperationError {}

impl OperationError {
    pub(crate) fn new(kind: ApiErrorKind, msg: &str) -> Self {
        OperationError::Service {
            kind,
            msg: msg.to_owned(),
        }
    }
}

impl From<ValidationError> for OperationError {
    fn from(value: ValidationError) -> Self {
        OperationError::Validation(value)
    }
}

impl From<sqlx::Error> for OperationError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::Database(ref db_error) => {
                if db_error.kind() == sqlx::error::ErrorKind::UniqueViolation {
                    OperationError::Service {
                        kind: ApiErrorKind::EntityAlreadyExists,
                        msg: error.to_string(),
                    }
                } else {
                    OperationError::Service {
                        kind: ApiErrorKind::ServiceFailure,
                        msg: error.to_string(),
                    }
                }
            }
            _ => OperationError::Service {
                kind: ApiErrorKind::ServiceFailure,
                msg: error.to_string(),
            },
        }
    }
}
