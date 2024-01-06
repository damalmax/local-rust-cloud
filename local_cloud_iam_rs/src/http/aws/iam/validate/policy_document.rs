use validator::Validate;

use local_cloud_iam_policy_document::types::LocalPolicyDocument;

use crate::http::aws::iam::actions::error::ApiErrorKind;
use crate::http::aws::iam::constants;
use crate::http::aws::iam::operations::error::OperationError;

pub(crate) fn validate(policy_document_str: &str) -> Result<String, OperationError> {
    let policy_document: LocalPolicyDocument = serde_json::from_str(policy_document_str)
        .map_err(|_err| OperationError::new(ApiErrorKind::MalformedPolicyDocument, "Malformed policy document."))?;
    policy_document
        .validate()
        .map_err(|_err| OperationError::new(ApiErrorKind::MalformedPolicyDocument, "Malformed policy document."))?;

    let json = serde_json::to_string(&policy_document)
        .map_err(|_err| OperationError::new(ApiErrorKind::ServiceFailure, "Failed to minimize Policy Document."))?;
    Ok(json)
}

pub(crate) fn validate_and_minify_managed(policy_document: &str) -> Result<String, OperationError> {
    let json = validate(policy_document)?;

    let length = json.chars().count();
    if json.chars().count() > constants::policy::MANAGED_POLICY_MAX_SIZE {
        return Err(OperationError::new(
            ApiErrorKind::MalformedPolicyDocument,
            format!(
                "Managed Policy Document length is greater ({length} characters) than allowed (max: {} characters)",
                constants::policy::MANAGED_POLICY_MAX_SIZE
            )
            .as_str(),
        ));
    }
    Ok(json)
}
