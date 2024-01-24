use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum ValidationErrorKind {
    Required,
    Min,
    Max,
    SizeMin,
    SizeMax,
    LengthMin,
    LengthMax,
    RegExp,
    Allowed,
    Other,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ValidationError {
    pub kind: ValidationErrorKind,
    pub message: String,
}

impl ValidationError {
    pub fn new(kind: ValidationErrorKind, message: impl Into<String>) -> Self {
        ValidationError {
            kind,
            message: message.into(),
        }
    }
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ValidationError {}
