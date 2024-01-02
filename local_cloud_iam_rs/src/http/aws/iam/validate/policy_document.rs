use validator::Validate;

use local_cloud_iam_policy_document::types::LocalPolicyDocument;

use crate::http::aws::iam::constants;
use crate::http::aws::iam::validate::error::{ValidationError, ValidationErrorKind};

pub(crate) fn validate(policy_document_str: &str) -> Result<String, ValidationError> {
    let policy_document: LocalPolicyDocument = serde_json::from_str(policy_document_str).map_err(|_err| {
        ValidationError::new(ValidationErrorKind::MalformedPolicyDocument, "Malformed policy document.")
    })?;
    policy_document.validate().map_err(|_err| {
        ValidationError::new(ValidationErrorKind::MalformedPolicyDocument, "Malformed policy document.")
    })?;

    let json = serde_json::to_string(&policy_document).map_err(|_err| {
        ValidationError::new(ValidationErrorKind::ServiceFailure, "Failed to minimize Policy Document.")
    })?;
    Ok(json)
}

pub(crate) fn validate_and_minify_managed(policy_document: &str) -> Result<String, ValidationError> {
    let json = validate(policy_document)?;

    let length = json.chars().count();
    if json.chars().count() > constants::policy::MANAGED_POLICY_MAX_SIZE {
        return Err(ValidationError::new(
            ValidationErrorKind::MalformedPolicyDocument,
            format!(
                "Managed Policy Document length is greater ({length} characters) than allowed (max: {} characters)",
                constants::policy::MANAGED_POLICY_MAX_SIZE
            ),
        ));
    }
    Ok(json)
}
