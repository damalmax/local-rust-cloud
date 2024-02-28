use aws_sdk_iam::operation::create_access_key::CreateAccessKeyOutput;
use aws_sdk_iam::operation::delete_access_key::DeleteAccessKeyOutput;
use aws_sdk_iam::operation::list_access_keys::ListAccessKeysOutput;
use aws_sdk_iam::operation::update_access_key::UpdateAccessKeyOutput;

use local_cloud_db::LocalDb;
use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::types::create_access_key::CreateAccessKeyRequest;
use crate::http::aws::iam::types::delete_access_key::DeleteAccessKeyRequest;
use crate::http::aws::iam::types::list_access_keys::ListAccessKeysRequest;
use crate::http::aws::iam::types::update_access_key::UpdateAccessKeyRequest;

pub(crate) async fn create_access_key(
    ctx: &OperationCtx, input: &CreateAccessKeyRequest, db: &LocalDb,
) -> Result<CreateAccessKeyOutput, OperationError> {
    input.validate("$")?;

    let output = CreateAccessKeyOutput::builder().build();

    Ok(output)
}

pub(crate) async fn update_access_key(
    ctx: &OperationCtx, input: &UpdateAccessKeyRequest, db: &LocalDb,
) -> Result<UpdateAccessKeyOutput, OperationError> {
    input.validate("$")?;

    let output = UpdateAccessKeyOutput::builder().build();

    Ok(output)
}

pub(crate) async fn list_access_keys(
    ctx: &OperationCtx, input: &ListAccessKeysRequest, db: &LocalDb,
) -> Result<ListAccessKeysOutput, OperationError> {
    input.validate("$")?;

    let output = ListAccessKeysOutput::builder().build().unwrap();

    Ok(output)
}

pub(crate) async fn delete_access_key(
    ctx: &OperationCtx, input: &DeleteAccessKeyRequest, db: &LocalDb,
) -> Result<DeleteAccessKeyOutput, OperationError> {
    input.validate("$")?;

    let output = DeleteAccessKeyOutput::builder().build();

    Ok(output)
}
