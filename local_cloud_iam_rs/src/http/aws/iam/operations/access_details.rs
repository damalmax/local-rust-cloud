use aws_sdk_iam::operation::generate_service_last_accessed_details::GenerateServiceLastAccessedDetailsOutput;
use aws_sdk_iam::operation::get_access_key_last_used::GetAccessKeyLastUsedOutput;

use local_cloud_db::LocalDb;
use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::types::generate_service_last_accessed_details::GenerateServiceLastAccessedDetailsRequest;
use crate::http::aws::iam::types::get_access_key_last_used::GetAccessKeyLastUsedRequest;

pub(crate) async fn generate_service_last_accessed_details(
    ctx: &OperationCtx, input: &GenerateServiceLastAccessedDetailsRequest, db: &LocalDb,
) -> Result<GenerateServiceLastAccessedDetailsOutput, OperationError> {
    input.validate("$")?;

    let output = GenerateServiceLastAccessedDetailsOutput::builder().build();

    Ok(output)
}

pub(crate) async fn get_access_key_last_used(
    ctx: &OperationCtx, input: &GetAccessKeyLastUsedRequest, db: &LocalDb,
) -> Result<GetAccessKeyLastUsedOutput, OperationError> {
    input.validate("$")?;

    let output = GetAccessKeyLastUsedOutput::builder().build();

    Ok(output)
}
