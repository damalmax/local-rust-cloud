use aws_sdk_iam::operation::create_service_specific_credential::CreateServiceSpecificCredentialOutput;
use aws_sdk_iam::operation::delete_service_specific_credential::DeleteServiceSpecificCredentialOutput;
use aws_sdk_iam::operation::list_service_specific_credentials::ListServiceSpecificCredentialsOutput;
use aws_sdk_iam::operation::reset_service_specific_credential::ResetServiceSpecificCredentialOutput;
use aws_sdk_iam::operation::update_service_specific_credential::UpdateServiceSpecificCredentialOutput;
use sqlx::{Sqlite, Transaction};

use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::ActionError;
use crate::http::aws::iam::types::create_service_specific_credential::CreateServiceSpecificCredentialRequest;
use crate::http::aws::iam::types::delete_service_specific_credential::DeleteServiceSpecificCredentialRequest;
use crate::http::aws::iam::types::list_service_specific_credentials::ListServiceSpecificCredentialsRequest;
use crate::http::aws::iam::types::reset_service_specific_credential::ResetServiceSpecificCredentialRequest;
use crate::http::aws::iam::types::update_service_specific_credential::UpdateServiceSpecificCredentialRequest;

pub(crate) async fn update_service_specific_credential<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &UpdateServiceSpecificCredentialRequest,
) -> Result<UpdateServiceSpecificCredentialOutput, ActionError> {
    input.validate("$")?;

    let output = UpdateServiceSpecificCredentialOutput::builder().build();
    Ok(output)
}

pub(crate) async fn create_service_specific_credential<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &CreateServiceSpecificCredentialRequest,
) -> Result<CreateServiceSpecificCredentialOutput, ActionError> {
    input.validate("$")?;

    let output = CreateServiceSpecificCredentialOutput::builder().build();
    Ok(output)
}

pub(crate) async fn list_service_specific_credentials<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &ListServiceSpecificCredentialsRequest,
) -> Result<ListServiceSpecificCredentialsOutput, ActionError> {
    input.validate("$")?;

    let output = ListServiceSpecificCredentialsOutput::builder().build();
    Ok(output)
}

pub(crate) async fn reset_service_specific_credential<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &ResetServiceSpecificCredentialRequest,
) -> Result<ResetServiceSpecificCredentialOutput, ActionError> {
    input.validate("$")?;

    let output = ResetServiceSpecificCredentialOutput::builder().build();
    Ok(output)
}

pub(crate) async fn delete_service_specific_credential<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &DeleteServiceSpecificCredentialRequest,
) -> Result<DeleteServiceSpecificCredentialOutput, ActionError> {
    input.validate("$")?;

    let output = DeleteServiceSpecificCredentialOutput::builder().build();
    Ok(output)
}
