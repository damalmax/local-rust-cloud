use aws_sdk_iam::operation::create_service_linked_role::CreateServiceLinkedRoleOutput;
use aws_sdk_iam::operation::delete_service_linked_role::DeleteServiceLinkedRoleOutput;
use aws_sdk_iam::operation::get_service_linked_role_deletion_status::GetServiceLinkedRoleDeletionStatusOutput;

use local_cloud_db::LocalDb;
use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::types::create_service_linked_role::CreateServiceLinkedRoleRequest;
use crate::http::aws::iam::types::delete_service_linked_role::DeleteServiceLinkedRoleRequest;
use crate::http::aws::iam::types::get_service_linked_role_deletion_status::GetServiceLinkedRoleDeletionStatusRequest;

pub(crate) async fn create_service_linked_role(
    ctx: &OperationCtx, input: &CreateServiceLinkedRoleRequest, db: &LocalDb,
) -> Result<CreateServiceLinkedRoleOutput, OperationError> {
    input.validate("$")?;

    let output = CreateServiceLinkedRoleOutput::builder().build();

    Ok(output)
}

pub(crate) async fn get_service_linked_role_deletion_status(
    ctx: &OperationCtx, input: &GetServiceLinkedRoleDeletionStatusRequest, db: &LocalDb,
) -> Result<GetServiceLinkedRoleDeletionStatusOutput, OperationError> {
    input.validate("$")?;

    let output = GetServiceLinkedRoleDeletionStatusOutput::builder().build().unwrap();

    Ok(output)
}

pub(crate) async fn delete_service_linked_role(
    ctx: &OperationCtx, input: &DeleteServiceLinkedRoleRequest, db: &LocalDb,
) -> Result<DeleteServiceLinkedRoleOutput, OperationError> {
    input.validate("$")?;

    let output = DeleteServiceLinkedRoleOutput::builder().build().unwrap();

    Ok(output)
}
