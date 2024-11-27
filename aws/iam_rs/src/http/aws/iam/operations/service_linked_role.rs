use aws_sdk_iam::operation::create_service_linked_role::CreateServiceLinkedRoleOutput;
use aws_sdk_iam::operation::delete_service_linked_role::DeleteServiceLinkedRoleOutput;
use aws_sdk_iam::operation::get_service_linked_role_deletion_status::GetServiceLinkedRoleDeletionStatusOutput;
use sqlx::{Sqlite, Transaction};

use validators::NamedValidator;

use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::ActionError;
use crate::http::aws::iam::types::create_service_linked_role::CreateServiceLinkedRoleRequest;
use crate::http::aws::iam::types::delete_service_linked_role::DeleteServiceLinkedRoleRequest;
use crate::http::aws::iam::types::get_service_linked_role_deletion_status::GetServiceLinkedRoleDeletionStatusRequest;

pub(crate) async fn create_service_linked_role<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &CreateServiceLinkedRoleRequest,
) -> Result<CreateServiceLinkedRoleOutput, ActionError> {
    input.validate("$")?;

    let output = CreateServiceLinkedRoleOutput::builder().build();
    Ok(output)
}

pub(crate) async fn get_service_linked_role_deletion_status<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &GetServiceLinkedRoleDeletionStatusRequest,
) -> Result<GetServiceLinkedRoleDeletionStatusOutput, ActionError> {
    input.validate("$")?;

    let output = GetServiceLinkedRoleDeletionStatusOutput::builder().build().unwrap();
    Ok(output)
}

pub(crate) async fn delete_service_linked_role<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &DeleteServiceLinkedRoleRequest,
) -> Result<DeleteServiceLinkedRoleOutput, ActionError> {
    input.validate("$")?;

    let output = DeleteServiceLinkedRoleOutput::builder().build().unwrap();
    Ok(output)
}
