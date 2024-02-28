use aws_sdk_iam::operation::get_account_authorization_details::GetAccountAuthorizationDetailsOutput;
use aws_sdk_iam::operation::get_account_password_policy::GetAccountPasswordPolicyOutput;
use aws_sdk_iam::operation::get_account_summary::GetAccountSummaryOutput;

use local_cloud_db::LocalDb;
use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::types::get_account_authorization_details::GetAccountAuthorizationDetailsRequest;
use crate::http::aws::iam::types::get_account_password_policy::GetAccountPasswordPolicyRequest;
use crate::http::aws::iam::types::get_account_summary::GetAccountSummaryRequest;

pub(crate) async fn get_account_authorization_details(
    ctx: &OperationCtx, input: &GetAccountAuthorizationDetailsRequest, db: &LocalDb,
) -> Result<GetAccountAuthorizationDetailsOutput, OperationError> {
    input.validate("$")?;

    let output = GetAccountAuthorizationDetailsOutput::builder().build();

    Ok(output)
}

pub(crate) async fn get_account_password_policy(
    ctx: &OperationCtx, input: &GetAccountPasswordPolicyRequest, db: &LocalDb,
) -> Result<GetAccountPasswordPolicyOutput, OperationError> {
    input.validate("$")?;

    let output = GetAccountPasswordPolicyOutput::builder().build();

    Ok(output)
}

pub(crate) async fn get_account_summary(
    ctx: &OperationCtx, input: &GetAccountSummaryRequest, db: &LocalDb,
) -> Result<GetAccountSummaryOutput, OperationError> {
    input.validate("$")?;

    let output = GetAccountSummaryOutput::builder().build();

    Ok(output)
}
