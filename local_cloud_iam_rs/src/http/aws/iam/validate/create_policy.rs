use crate::http::aws::iam;
use crate::http::aws::iam::actions::create_policy::LocalCreatePolicy;
use crate::http::aws::iam::constants;
use crate::http::aws::iam::validate::error::{ValidationError, ValidationErrorKind};
use crate::http::aws::iam::validate::policy::{validate_path, validate_policy_name};

pub(crate) fn validate(input: &LocalCreatePolicy) -> Result<(), ValidationError> {
    validate_policy_name(input.policy_name())?;
    validate_tags(input)?;
    validate_policy_document_present(input)?;
    validate_path(input.path.as_deref())?;
    Ok(())
}

fn validate_tags(input: &LocalCreatePolicy) -> Result<(), ValidationError> {
    match input.tags() {
        None => Ok(()),
        Some(tags) => {
            if tags.len() > constants::tag::SESSION_TAGS_MAX_COUNT {
                return Err(ValidationError::new(
                    ValidationErrorKind::InvalidInput,
                    format!(
                        "The number of submitted tags is larger ({} tags) than allowed (limit: {} tags).",
                        tags.len(),
                        constants::tag::SESSION_TAGS_MAX_COUNT
                    ),
                ));
            }
            for (id, tag) in tags.iter().enumerate() {
                iam::validate::tag::validate_key(tag.key.as_deref(), id)?;
                iam::validate::tag::validate_value(tag.value.as_deref(), id)?;
            }
            Ok(())
        }
    }
}

fn validate_policy_document_present(input: &LocalCreatePolicy) -> Result<(), ValidationError> {
    match input.policy_document() {
        None => Err(ValidationError::new(ValidationErrorKind::InvalidInput, "Policy Document is not provided.")),
        Some(_) => Ok(()),
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::http::aws::iam::actions::create_policy::LocalCreatePolicy;
    use crate::http::aws::iam::validate::create_policy::validate_policy_document_present;
    use crate::http::aws::iam::validate::error::{ValidationError, ValidationErrorKind};

    #[test]
    fn test_validate_policy_document_present_not_provided() {
        let input = LocalCreatePolicy {
            policy_name: None,
            path: None,
            policy_document: None,
            description: None,
            tags: None,
            attachable: None,
        };

        let result = validate_policy_document_present(&input);
        assert!(result.is_err());
        let expected_error =
            ValidationError::new(ValidationErrorKind::InvalidInput, "Policy Document is not provided.");
        assert_eq!(result.unwrap_err(), expected_error);
    }

    #[test]
    fn test_validate_policy_document_present_present() {
        let input = LocalCreatePolicy {
            policy_name: None,
            path: None,
            policy_document: Some(
                json!({
                    "Version":"2012-10-17",
                    "Statement":[]
                })
                .to_string(),
            ),
            description: None,
            tags: None,
            attachable: None,
        };

        let result = validate_policy_document_present(&input);
        assert!(result.is_ok());
    }
}
