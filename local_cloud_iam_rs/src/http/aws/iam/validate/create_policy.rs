use crate::http::aws::iam;
use crate::http::aws::iam::actions::create_policy::LocalCreatePolicy;
use crate::http::aws::iam::constants;
use crate::http::aws::iam::validate::common::validate_property_present;
use crate::http::aws::iam::validate::error::{ValidationError, ValidationErrorKind};
use crate::http::aws::iam::validate::policy::{validate_path, validate_policy_name};

pub(crate) fn validate(input: &LocalCreatePolicy) -> Result<(), ValidationError> {
    validate_policy_name(input.policy_name())?;
    validate_tags(input)?;
    validate_property_present(input.policy_document(), || "PolicyDocument is not provided.")?;
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
