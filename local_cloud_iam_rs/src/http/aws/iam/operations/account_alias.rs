use aws_sdk_iam::operation::create_account_alias::CreateAccountAliasOutput;
use aws_sdk_iam::operation::delete_account_alias::DeleteAccountAliasOutput;
use aws_sdk_iam::operation::list_account_aliases::ListAccountAliasesOutput;

use local_cloud_db::LocalDb;
use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::types::create_account_alias::CreateAccountAliasRequest;
use crate::http::aws::iam::types::delete_account_alias::DeleteAccountAliasRequest;
use crate::http::aws::iam::types::list_account_aliases::ListAccountAliasesRequest;

pub(crate) async fn create_account_alias(
    ctx: &OperationCtx, input: &CreateAccountAliasRequest, db: &LocalDb,
) -> Result<CreateAccountAliasOutput, OperationError> {
    input.validate("$")?;

    let output = CreateAccountAliasOutput::builder().build();

    Ok(output)
}

pub(crate) async fn list_account_aliases(
    ctx: &OperationCtx, input: &ListAccountAliasesRequest, db: &LocalDb,
) -> Result<ListAccountAliasesOutput, OperationError> {
    input.validate("$")?;

    let output = ListAccountAliasesOutput::builder().build().unwrap();

    Ok(output)
}

pub(crate) async fn delete_account_alias(
    ctx: &OperationCtx, input: &DeleteAccountAliasRequest, db: &LocalDb,
) -> Result<DeleteAccountAliasOutput, OperationError> {
    input.validate("$")?;

    let output = DeleteAccountAliasOutput::builder().build();

    Ok(output)
}
