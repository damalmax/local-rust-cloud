use crate::http::aws::iam::actions::create_policy_version::LocalCreatePolicyVersion;
use crate::http::aws::iam::validate::arn::validate_arn;
use crate::http::aws::iam::validate::common::validate_property_present;
use crate::http::aws::iam::validate::error::ValidationError;

pub(crate) fn validate(input: &LocalCreatePolicyVersion) -> Result<(), ValidationError> {
    validate_property_present(input.policy_arn(), || "PolicyArn is not provided.")?;
    validate_arn(input.policy_arn())?;
    validate_property_present(input.policy_document(), || "PolicyDocument is not provided.")?;
    Ok(())
}
