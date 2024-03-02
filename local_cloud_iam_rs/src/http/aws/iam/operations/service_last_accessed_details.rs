use aws_sdk_iam::operation::get_service_last_accessed_details::GetServiceLastAccessedDetailsOutput;
use aws_sdk_iam::operation::get_service_last_accessed_details_with_entities::GetServiceLastAccessedDetailsWithEntitiesOutput;
use sqlx::{Sqlite, Transaction};

use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::ActionError;
use crate::http::aws::iam::types::get_service_last_accessed_details::GetServiceLastAccessedDetailsRequest;
use crate::http::aws::iam::types::get_service_last_accessed_details_with_entities::GetServiceLastAccessedDetailsWithEntitiesRequest;

pub(crate) async fn get_service_last_accessed_details<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &GetServiceLastAccessedDetailsRequest,
) -> Result<GetServiceLastAccessedDetailsOutput, ActionError> {
    input.validate("$")?;

    let output = GetServiceLastAccessedDetailsOutput::builder().build().unwrap();
    Ok(output)
}

pub(crate) async fn get_service_last_accessed_details_with_entities<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &GetServiceLastAccessedDetailsWithEntitiesRequest,
) -> Result<GetServiceLastAccessedDetailsWithEntitiesOutput, ActionError> {
    input.validate("$")?;

    let output = GetServiceLastAccessedDetailsWithEntitiesOutput::builder()
        .build()
        .unwrap();
    Ok(output)
}
