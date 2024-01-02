use crate::http::aws::iam;
use crate::http::aws::iam::actions::create_policy::LocalCreatePolicy;
use crate::http::aws::iam::constants;
use crate::http::aws::iam::validate::error::{ValidationError, ValidationErrorKind};

pub(crate) fn validate(value: &LocalCreatePolicy) -> Result<(), ValidationError> {
    validate_policy_name(value)?;
    validate_tags(value)?;
    validate_policy_document_present(value)?;
    Ok(())
}

fn validate_tags(value: &LocalCreatePolicy) -> Result<(), ValidationError> {
    match value.tags() {
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
                iam::validate::tag::validate_key(tag, id)?;
                iam::validate::tag::validate_value(tag, id)?;
            }
            Ok(())
        }
    }
}

fn validate_policy_name(value: &LocalCreatePolicy) -> Result<(), ValidationError> {
    let policy_name = value.policy_name().unwrap_or("");
    if policy_name.trim().len() > 1 {
        return Ok(());
    }

    Ok(())
}

fn validate_policy_document_present(value: &LocalCreatePolicy) -> Result<(), ValidationError> {
    match value.policy_document() {
        None => Err(ValidationError::new(ValidationErrorKind::InvalidInput, "Policy Document is not provided.")),
        Some(_) => Ok(()),
    }
}
