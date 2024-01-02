use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ValidationErrorKind {
    /// Type of an error thrown because of invalid request data submitted to the service.
    InvalidInput,
    /// Type of an error thrown because of malformed policy document.
    MalformedPolicyDocument,
    /// Type of unexpected service error.
    ServiceFailure,
}

#[derive(Debug)]
pub(crate) struct ValidationError {
    pub(crate) kind: ValidationErrorKind,
    pub(crate) message: String,
}

impl ValidationError {
    pub(crate) fn new(kind: ValidationErrorKind, message: impl Into<String>) -> Self {
        ValidationError {
            kind,
            message: message.into(),
        }
    }
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Validation error: {}", self.message)
    }
}

impl Error for ValidationError {}
