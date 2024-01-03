use std::iter::{IntoIterator, Iterator};

use crate::http::aws::iam::constants;
use crate::http::aws::iam::validate::error::{ValidationError, ValidationErrorKind};

fn is_valid_input(input: &str, valid_characters: &[char]) -> bool {
    input.chars().into_iter().all(|ch| valid_characters.contains(&ch))
}

pub(crate) fn validate_policy_name(value: Option<&str>) -> Result<(), ValidationError> {
    let policy_name = value.unwrap_or("").trim();
    let length = policy_name.chars().count();
    if length > constants::policy::POLICY_NAME_MAX_SIZE {
        return Err(ValidationError::new(
            ValidationErrorKind::InvalidInput,
            format!(
                "The PolicyName length is larger ({length} characters) than allowed (max: {} characters).",
                constants::policy::POLICY_NAME_MAX_SIZE
            ),
        ));
    }
    if length < constants::policy::POLICY_NAME_MIN_SIZE {
        return Err(ValidationError::new(
            ValidationErrorKind::InvalidInput,
            format!(
                "The PolicyName length is less ({length} characters) than allowed (min: {} character(s)).",
                constants::policy::POLICY_NAME_MIN_SIZE
            ),
        ));
    }
    if !is_valid_input(policy_name, &constants::policy::POLICY_NAME_VALID_CHARACTERS) {
        return Err(ValidationError::new(
            ValidationErrorKind::InvalidInput,
            "Please use alphanumeric and '+=,.@-_' characters for PolicyName.",
        ));
    }

    Ok(())
}

pub(crate) fn validate_policy_description(value: Option<&str>) -> Result<(), ValidationError> {
    match value {
        None => Ok(()),
        Some(policy_description) => {
            let policy_description = policy_description.trim();
            let length = policy_description.chars().count();
            if length > constants::policy::POLICY_DESCRIPTION_MAX_SIZE {
                return Err(ValidationError::new(
                    ValidationErrorKind::InvalidInput,
                    format!(
                        "The Description length is larger ({length} characters) than allowed (max: {} characters).",
                        constants::policy::POLICY_DESCRIPTION_MAX_SIZE
                    ),
                ));
            }
            if !is_valid_input(policy_description, &constants::policy::POLICY_DESCRIPTION_VALID_CHARACTERS) {
                return Err(ValidationError::new(
                    ValidationErrorKind::InvalidInput,
                    "Please use alphanumeric, whitespaces and '+=,.@-_' characters for the Description.",
                ));
            }

            Ok(())
        }
    }
}

pub(crate) fn validate_path(value: Option<&str>) -> Result<(), ValidationError> {
    match value {
        None => Ok(()),
        Some(path) => {
            let path = path.trim();
            let length = path.chars().count();
            if length > constants::policy::PATH_MAX_SIZE {
                return Err(ValidationError::new(
                    ValidationErrorKind::InvalidInput,
                    format!(
                        "The Path length is larger ({length} characters) than allowed (max: {} characters).",
                        constants::policy::PATH_MAX_SIZE
                    ),
                ));
            }
            if !constants::policy::POLICY_PATH_REGEX.is_match(path) {
                return Err(ValidationError::new(
                    ValidationErrorKind::InvalidInput,
                    format!(
                        "The Path must follow the following pattern: '{}'.",
                        constants::policy::POLICY_PATH_REGEX.as_str()
                    ),
                ));
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use local_cloud_common::naming::generate_char_sequence;

    use crate::http::aws::iam::validate::error::{ValidationError, ValidationErrorKind};
    use crate::http::aws::iam::validate::policy::{
        constants::policy::POLICY_DESCRIPTION_VALID_CHARACTERS, validate_path, validate_policy_description,
        validate_policy_name,
    };

    #[rstest]
    #[case(
        None,
        "The PolicyName length is less (0 characters) than allowed (min: 1 character(s))."
    )]
    #[case(
        Some(""),
        "The PolicyName length is less (0 characters) than allowed (min: 1 character(s))."
    )]
    #[case(
        Some("2g784dfqwertyuiopasdfghjklmnbvcxz1234567890qwertyuioasdgfjgkdownzjwuwfgfwgfhjwegfwegfwefgwejhfgwhejgfwejhfgwejgfwej34567qwertyuio"),
        "The PolicyName length is larger (129 characters) than allowed (max: 128 characters)."
    )]
    #[case(Some("&test"), "Please use alphanumeric and '+=,.@-_' characters for PolicyName.")]
    fn test_validate_policy_name_invalid(#[case] policy_name: Option<&str>, #[case] expected_error: &str) {
        let error = ValidationError::new(ValidationErrorKind::InvalidInput, expected_error);

        let result = validate_policy_name(policy_name);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), error);
    }

    #[test]
    fn test_validate_policy_description_invalid_length() {
        let chars: Vec<u8> = POLICY_DESCRIPTION_VALID_CHARACTERS
            .to_owned()
            .clone()
            .into_iter()
            .map(|ch| ch as u8)
            .collect();
        let policy_description_text = generate_char_sequence(&chars, 1001);
        let policy_description = Some(policy_description_text.as_str());
        let error = ValidationError::new(
            ValidationErrorKind::InvalidInput,
            "The Description length is larger (1001 characters) than allowed (max: 1000 characters).",
        );

        let result = validate_policy_description(policy_description);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), error);
    }

    #[rstest]
    #[case(
        Some("&test"),
        "Please use alphanumeric, whitespaces and '+=,.@-_' characters for the Description."
    )]
    fn test_validate_policy_description_invalid_characters(
        #[case] policy_description: Option<&str>, #[case] expected_error: &str,
    ) {
        let error = ValidationError::new(ValidationErrorKind::InvalidInput, expected_error);

        let result = validate_policy_description(policy_description);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), error);
    }

    #[rstest]
    #[case(None)]
    #[case(Some(""))]
    #[case(Some("test"))]
    #[case(Some("test description"))]
    #[case(Some("test_description 12345"))]
    #[case(Some("test_description 12345 +=,.@-_"))]
    fn test_validate_policy_description_valid_characters(#[case] policy_description: Option<&str>) {
        let result = validate_policy_description(policy_description);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_path_invalid_length() {
        let chars: Vec<u8> = local_cloud_common::naming::ALPHANUMERIC_CAPITALIZED_CHARSET
            .to_owned()
            .into_iter()
            .collect();
        let path_text = String::from("/") + generate_char_sequence(&chars, 512).as_str();
        let path = Some(path_text.as_str());
        let error = ValidationError::new(
            ValidationErrorKind::InvalidInput,
            "The Path length is larger (513 characters) than allowed (max: 512 characters).",
        );

        let result = validate_path(path);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), error);
    }

    #[rstest]
    #[case(
        Some("a"),
        "The Path must follow the following pattern: '^((/[A-Za-z0-9\\.,\\+@=_-]+)*)/$'."
    )]
    #[case(
        Some("/qwerty"),
        "The Path must follow the following pattern: '^((/[A-Za-z0-9\\.,\\+@=_-]+)*)/$'."
    )]
    #[case(
        Some("/qwer&ty/"),
        "The Path must follow the following pattern: '^((/[A-Za-z0-9\\.,\\+@=_-]+)*)/$'."
    )]
    fn test_validate_path_invalid_characters(#[case] path: Option<&str>, #[case] expected_error: &str) {
        let error = ValidationError::new(ValidationErrorKind::InvalidInput, expected_error);

        let result = validate_path(path);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), error);
    }

    #[test]
    fn test_validate_path_valid() {
        let result = validate_path(Some("/qwerty/"));
        assert!(result.is_ok());
    }
}
