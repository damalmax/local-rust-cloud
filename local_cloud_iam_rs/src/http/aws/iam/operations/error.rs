use std::error::Error;
use std::fmt::{Display, Formatter};

use local_cloud_validate::ValidationError;

use crate::http::aws::iam::actions::error::ApiErrorKind;

#[derive(Debug)]
pub(crate) enum ActionError {
    Service { kind: ApiErrorKind, msg: String },
    Validation(ValidationError),
}

impl Display for ActionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionError::Service { kind, msg } => {
                writeln!(f, "Action Error. Kind: {}, Error Message: {}", kind, msg)
            }
            ActionError::Validation(error) => error.fmt(f),
        }
    }
}

impl Error for ActionError {}

impl ActionError {
    pub(crate) fn new(kind: ApiErrorKind, msg: &str) -> Self {
        ActionError::Service {
            kind,
            msg: msg.to_owned(),
        }
    }
}

impl From<ValidationError> for ActionError {
    fn from(value: ValidationError) -> Self {
        ActionError::Validation(value)
    }
}

impl From<sqlx::Error> for ActionError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::Database(ref db_error) => {
                if db_error.kind() == sqlx::error::ErrorKind::UniqueViolation {
                    ActionError::Service {
                        kind: ApiErrorKind::EntityAlreadyExists,
                        msg: "Entity already exists.".to_owned(),
                    }
                } else {
                    ActionError::Service {
                        kind: ApiErrorKind::ServiceFailure,
                        msg: error.to_string(),
                    }
                }
            }
            _ => ActionError::Service {
                kind: ApiErrorKind::ServiceFailure,
                msg: error.to_string(),
            },
        }
    }
}
