use aws_sdk_iam::operation::create_policy::CreatePolicyInput;

use crate::error::IamError;

pub fn validate(input: &CreatePolicyInput) -> Result<(), IamError> {
    Result::Ok(())
}
