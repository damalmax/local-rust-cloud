use aws_sdk_iam::operation::delete_account_password_policy::DeleteAccountPasswordPolicyOutput;
use aws_sdk_iam::operation::update_account_password_policy::UpdateAccountPasswordPolicyOutput;
use sqlx::{Sqlite, Transaction};

use validators::NamedValidator;

use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::ActionError;
use crate::http::aws::iam::types::delete_account_password_policy::DeleteAccountPasswordPolicyRequest;
use crate::http::aws::iam::types::update_account_password_policy::UpdateAccountPasswordPolicyRequest;

pub(crate) async fn update_account_password_policy<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &UpdateAccountPasswordPolicyRequest,
) -> Result<UpdateAccountPasswordPolicyOutput, ActionError> {
    input.validate("$")?;

    let output = UpdateAccountPasswordPolicyOutput::builder().build();
    Ok(output)
}

pub(crate) async fn delete_account_password_policy<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &DeleteAccountPasswordPolicyRequest,
) -> Result<DeleteAccountPasswordPolicyOutput, ActionError> {
    input.validate("$")?;

    let output = DeleteAccountPasswordPolicyOutput::builder().build();
    Ok(output)
}
