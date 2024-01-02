use regex::Regex;

use crate::http::aws::iam::actions::tag::LocalTag;
use crate::http::aws::iam::constants;
use crate::http::aws::iam::validate::error::{ValidationError, ValidationErrorKind};

pub(crate) fn validate_key(tag: &LocalTag, tag_index: usize) -> Result<(), ValidationError> {
    match &tag.key {
        None => Err(ValidationError::new(
            ValidationErrorKind::InvalidInput,
            format!("No key for tag provided. Location: 'Tags.member.{tag_index}.key'"),
        )),
        Some(key) => {
            let length = key.chars().count();
            if length < constants::tag::TAG_KEY_MIN_SIZE {
                Err(ValidationError::new(ValidationErrorKind::InvalidInput,
                                         format!("Tag key length is less ({length} characters) than allowed (min: {} characters). Location: 'Tags.member.{tag_index}.key'.", constants::tag::TAG_KEY_MIN_SIZE)))
            } else if length > constants::tag::TAG_KEY_MAX_SIZE {
                Err(ValidationError::new(ValidationErrorKind::InvalidInput, format!("Tag key length is greater ({length} characters) than allowed (max: {} characters). Location: 'Tags.member.{tag_index}.key'.", constants::tag::TAG_KEY_MAX_SIZE)))
            } else {
                validate_with_regexp(
                    key,
                    &constants::tag::TAG_KEY_REGEX,
                    format!(
                        "Tag key must follow the following RegExp: '{}', Location: 'Tags.member.{tag_index}.key'.",
                        constants::tag::TAG_KEY_REGEX.as_str()
                    )
                    .as_str(),
                )
            }
        }
    }
}

pub(crate) fn validate_value(tag: &LocalTag, tag_index: usize) -> Result<(), ValidationError> {
    match &tag.value {
        None => Err(ValidationError::new(
            ValidationErrorKind::InvalidInput,
            format!("No value for tag provided. Location: 'Tags.member.{tag_index}.value'"),
        )),
        Some(value) => {
            let length = value.chars().count();
            if length < constants::tag::TAG_VALUE_MIN_SIZE {
                Err(ValidationError::new(ValidationErrorKind::InvalidInput,
                                         format!("Tag value length is less ({length} characters) than allowed (min: {} characters). Location: 'Tags.member.{tag_index}.value'.", constants::tag::TAG_VALUE_MIN_SIZE)))
            } else if length > constants::tag::TAG_VALUE_MAX_SIZE {
                Err(ValidationError::new(ValidationErrorKind::InvalidInput, format!("Tag value length is greater ({length} characters) than allowed (max: {} characters). Location: 'Tags.member.{tag_index}.value'.", constants::tag::TAG_VALUE_MAX_SIZE)))
            } else {
                validate_with_regexp(
                    value,
                    &constants::tag::TAG_VALUE_REGEX,
                    format!(
                        "Tag value must follow the following RegExp: '{}', Location: 'Tags.member.{tag_index}.value'.",
                        constants::tag::TAG_VALUE_REGEX.as_str()
                    )
                    .as_str(),
                )
            }
        }
    }
}

fn validate_with_regexp(value_to_validate: &str, regex: &Regex, message: &str) -> Result<(), ValidationError> {
    if regex.is_match(value_to_validate) {
        Ok(())
    } else {
        Err(ValidationError::new(ValidationErrorKind::InvalidInput, message))
    }
}
