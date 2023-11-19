use aws_sdk_iam::operation::create_policy::CreatePolicyInput;

use crate::aws::actions::errors::IamApiError;

use super::tags;

pub fn validate(request_id: &str, input: &CreatePolicyInput) -> Result<(), IamApiError> {
    tags::validate(request_id, input.tags())?;
    Result::Ok(())
}
