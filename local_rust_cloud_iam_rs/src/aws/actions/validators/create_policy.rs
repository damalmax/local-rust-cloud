use crate::aws::actions::{create_policy_request::LocalCreatePolicyInput, errors::IamApiError};

use super::tags;

pub fn validate(request_id: &str, input: &LocalCreatePolicyInput) -> Result<(), IamApiError> {
    tags::validate(request_id, input.local_tags())?;
    Result::Ok(())
}
