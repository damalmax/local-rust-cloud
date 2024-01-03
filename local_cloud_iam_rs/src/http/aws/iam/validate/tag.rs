use regex::Regex;

use crate::http::aws::iam::constants;
use crate::http::aws::iam::validate::error::{ValidationError, ValidationErrorKind};

pub(crate) fn validate_key(tag_key: Option<&str>, tag_index: usize) -> Result<(), ValidationError> {
    match tag_key {
        None => Err(ValidationError::new(
            ValidationErrorKind::InvalidInput,
            format!("No key for tag provided. Location: 'Tags.member.{tag_index}.key'."),
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
                    &key,
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

pub(crate) fn validate_value(tag_value: Option<&str>, tag_index: usize) -> Result<(), ValidationError> {
    match tag_value {
        None => Err(ValidationError::new(
            ValidationErrorKind::InvalidInput,
            format!("No value for tag provided. Location: 'Tags.member.{tag_index}.value'."),
        )),
        Some(value) => {
            let length = value.chars().count();
            if length > constants::tag::TAG_VALUE_MAX_SIZE {
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

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use local_cloud_common::naming::generate_char_sequence;

    use crate::http::aws::iam::validate::error::{ValidationError, ValidationErrorKind};
    use crate::http::aws::iam::validate::tag::{validate_key, validate_value};

    #[rstest]
    #[case(None, 2, "No key for tag provided. Location: 'Tags.member.2.key'.")]
    #[case(
        Some(""),
        1,
        "Tag key length is less (0 characters) than allowed (min: 1 characters). Location: 'Tags.member.1.key'."
    )]
    #[case(
        Some("tag&key"),
        11,
        "Tag key must follow the following RegExp: '^[\\p{L}\\p{Z}\\p{N}_.:/=+\\-@]+$', Location: 'Tags.member.11.key'."
    )]
    fn test_validate_key__invalid_characters(
        #[case] tag_key: Option<&str>, #[case] index: usize, #[case] expected_error: &str,
    ) {
        let error = ValidationError::new(ValidationErrorKind::InvalidInput, expected_error);

        let result = validate_key(tag_key, index);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), error);
    }

    #[test]
    fn test_validate_key__invalid_length() {
        let error = ValidationError::new(ValidationErrorKind::InvalidInput, "Tag key length is greater (129 characters) than allowed (max: 128 characters). Location: 'Tags.member.3.key'.");

        let tag_key_text = generate_char_sequence(&local_cloud_common::naming::ALPHANUMERIC_CAPITALIZED_CHARSET, 129);
        let tag_key = Some(tag_key_text.as_str());

        let result = validate_key(tag_key, 3);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), error);
    }

    #[rstest]
    #[case(None, 2, "No value for tag provided. Location: 'Tags.member.2.value'.")]
    #[case(
    Some("tag&value"),
    11,
    "Tag value must follow the following RegExp: '^[\\p{L}\\p{Z}\\p{N}_.:/=+\\-@]*$', Location: 'Tags.member.11.value'."
    )]
    fn test_validate_value__invalid_characters(
        #[case] tag_value: Option<&str>, #[case] index: usize, #[case] expected_error: &str,
    ) {
        let error = ValidationError::new(ValidationErrorKind::InvalidInput, expected_error);

        let result = validate_value(tag_value, index);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), error);
    }

    #[test]
    fn test_validate_value__invalid_length() {
        let error = ValidationError::new(ValidationErrorKind::InvalidInput, "Tag value length is greater (257 characters) than allowed (max: 256 characters). Location: 'Tags.member.3.value'.");

        let tag_value_text = generate_char_sequence(&local_cloud_common::naming::ALPHANUMERIC_CAPITALIZED_CHARSET, 257);
        let tag_value = Some(tag_value_text.as_str());

        let result = validate_value(tag_value, 3);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), error);
    }
}
