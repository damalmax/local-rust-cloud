use std::fmt::{Display, Formatter};

use serde::de::StdError;

#[derive(Debug)]
pub enum DeError {
    InvalidSource(String),
    Custom(String),
    Internal(String),
    RootNode(String),
}

impl StdError for DeError {}

impl Display for DeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DeError::InvalidSource(msg) => {
                write!(f, "InvalidSource: {}", msg)
            }
            DeError::Custom(msg) => {
                write!(f, "{}", msg)
            }
            DeError::Internal(msg) => {
                write!(f, "InternalError: {}", msg)
            }
            DeError::RootNode(msg) => {
                write!(f, "deserializing '{}' at the root level is unsupported", msg)
            }
        }
    }
}

impl serde::de::Error for DeError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        DeError::Custom(msg.to_string())
    }
}
