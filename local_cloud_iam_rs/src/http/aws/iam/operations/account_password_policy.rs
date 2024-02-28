use aws_sdk_iam::operation::delete_account_password_policy::DeleteAccountPasswordPolicyOutput;
use aws_sdk_iam::operation::update_account_password_policy::UpdateAccountPasswordPolicyOutput;

use local_cloud_db::LocalDb;
use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::types::delete_account_password_policy::DeleteAccountPasswordPolicyRequest;
use crate::http::aws::iam::types::update_account_password_policy::UpdateAccountPasswordPolicyRequest;

pub(crate) async fn update_account_password_policy(
    ctx: &OperationCtx, input: &UpdateAccountPasswordPolicyRequest, db: &LocalDb,
) -> Result<UpdateAccountPasswordPolicyOutput, OperationError> {
    input.validate("$")?;

    let output = UpdateAccountPasswordPolicyOutput::builder().build();

    Ok(output)
}

pub(crate) async fn delete_account_password_policy(
    ctx: &OperationCtx, input: &DeleteAccountPasswordPolicyRequest, db: &LocalDb,
) -> Result<DeleteAccountPasswordPolicyOutput, OperationError> {
    input.validate("$")?;

    let output = DeleteAccountPasswordPolicyOutput::builder().build();

    Ok(output)
}
