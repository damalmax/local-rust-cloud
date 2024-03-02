use aws_sdk_iam::operation::generate_service_last_accessed_details::GenerateServiceLastAccessedDetailsOutput;
use aws_sdk_iam::operation::get_access_key_last_used::GetAccessKeyLastUsedOutput;
use sqlx::{Sqlite, Transaction};

use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::ActionError;
use crate::http::aws::iam::types::generate_service_last_accessed_details::GenerateServiceLastAccessedDetailsRequest;
use crate::http::aws::iam::types::get_access_key_last_used::GetAccessKeyLastUsedRequest;

pub(crate) async fn generate_service_last_accessed_details<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &GenerateServiceLastAccessedDetailsRequest,
) -> Result<GenerateServiceLastAccessedDetailsOutput, ActionError> {
    input.validate("$")?;

    let output = GenerateServiceLastAccessedDetailsOutput::builder().build();
    Ok(output)
}

pub(crate) async fn get_access_key_last_used<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &GetAccessKeyLastUsedRequest,
) -> Result<GetAccessKeyLastUsedOutput, ActionError> {
    input.validate("$")?;

    let output = GetAccessKeyLastUsedOutput::builder().build();
    Ok(output)
}
