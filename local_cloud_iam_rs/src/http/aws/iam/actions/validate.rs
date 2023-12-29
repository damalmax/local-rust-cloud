use derive_more::{Display, Error};

use crate::http::aws::iam::actions::error::IamApiError;

/// The trait provides base API for source validation. If structure needs to be validate,
/// it must implement this trait.
/// The intention of the validator is to use Fail-Fast approach.
pub(crate) trait IamValidator {
    fn validate(&self) -> Result<(), IamApiError>;
}

#[derive(Debug, Display, Error)]
pub(crate) enum ValidationError {
    #[display(fmt = "No tag key provided. Location: '{}'.", at)]
    TagNoKey { at: String },
    #[display(fmt = "No tag value provided. Location: '{}'.", at)]
    TagNoValue { at: String },
    #[display(
        fmt = "Tag key length is less ({} characters) than allowed (min: {} characters). Location: '{}'.",
        size,
        min,
        at
    )]
    TagKeyMinLength { size: usize, min: usize, at: String },
    #[display(
        fmt = "Tag key length is greater ({} characters) than allowed (max: {} characters). Location: '{}'.",
        size,
        max,
        at
    )]
    TagKeyMaxLength { size: usize, max: usize, at: String },
    #[display(
        fmt = "Tag value length is less ({} characters) than allowed (min: {} characters). Location: '{}'.",
        size,
        min,
        at
    )]
    TagValueMinLength { size: usize, min: usize, at: String },
    #[display(
        fmt = "Tag value length is greater ({} characters) than allowed (max: {} characters). Location: '{}'.",
        size,
        max,
        at
    )]
    TagValueMaxLength { size: usize, max: usize, at: String },
    #[display(
        fmt = "The number of submitted tags is larger ({} tags) than allowed (limit: {} tags).",
        count,
        max
    )]
    TooManyTags { count: usize, max: usize },
}
