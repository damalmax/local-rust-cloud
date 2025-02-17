use aws_sdk_iam::operation::get_account_authorization_details::GetAccountAuthorizationDetailsOutput;
use aws_sdk_iam::operation::get_account_password_policy::GetAccountPasswordPolicyOutput;
use aws_sdk_iam::operation::get_account_summary::GetAccountSummaryOutput;
use sqlx::{Sqlite, Transaction};

use validators::NamedValidator;

use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::ActionError;
use crate::http::aws::iam::types::get_account_authorization_details::GetAccountAuthorizationDetailsRequest;
use crate::http::aws::iam::types::get_account_password_policy::GetAccountPasswordPolicyRequest;
use crate::http::aws::iam::types::get_account_summary::GetAccountSummaryRequest;

pub(crate) async fn get_account_authorization_details<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &GetAccountAuthorizationDetailsRequest,
) -> Result<GetAccountAuthorizationDetailsOutput, ActionError> {
    input.validate("$")?;

    let output = GetAccountAuthorizationDetailsOutput::builder().build();
    Ok(output)
}

pub(crate) async fn get_account_password_policy<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &GetAccountPasswordPolicyRequest,
) -> Result<GetAccountPasswordPolicyOutput, ActionError> {
    input.validate("$")?;

    let output = GetAccountPasswordPolicyOutput::builder().build();
    Ok(output)
}

pub(crate) async fn get_account_summary<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &GetAccountSummaryRequest,
) -> Result<GetAccountSummaryOutput, ActionError> {
    input.validate("$")?;

    let output = GetAccountSummaryOutput::builder().build();
    Ok(output)
}
