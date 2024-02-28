use aws_sdk_iam::operation::get_service_last_accessed_details::GetServiceLastAccessedDetailsOutput;
use aws_sdk_iam::operation::get_service_last_accessed_details_with_entities::GetServiceLastAccessedDetailsWithEntitiesOutput;

use local_cloud_db::LocalDb;
use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::types::get_service_last_accessed_details::GetServiceLastAccessedDetailsRequest;
use crate::http::aws::iam::types::get_service_last_accessed_details_with_entities::GetServiceLastAccessedDetailsWithEntitiesRequest;

pub(crate) async fn get_service_last_accessed_details(
    ctx: &OperationCtx, input: &GetServiceLastAccessedDetailsRequest, db: &LocalDb,
) -> Result<GetServiceLastAccessedDetailsOutput, OperationError> {
    input.validate("$")?;

    let output = GetServiceLastAccessedDetailsOutput::builder().build().unwrap();

    Ok(output)
}

pub(crate) async fn get_service_last_accessed_details_with_entities(
    ctx: &OperationCtx, input: &GetServiceLastAccessedDetailsWithEntitiesRequest, db: &LocalDb,
) -> Result<GetServiceLastAccessedDetailsWithEntitiesOutput, OperationError> {
    input.validate("$")?;

    let output = GetServiceLastAccessedDetailsWithEntitiesOutput::builder()
        .build()
        .unwrap();

    Ok(output)
}
