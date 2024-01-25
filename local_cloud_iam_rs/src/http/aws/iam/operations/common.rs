use sqlx::{Sqlite, Transaction};

use crate::http::aws::iam::actions::error::ApiErrorKind;
use crate::http::aws::iam::db;
use crate::http::aws::iam::db::types::resource_identifier::{ResourceIdentifier, ResourceType};
use crate::http::aws::iam::operations::error::OperationError;

pub(crate) async fn create_resource_id<'a>(
    tx: &mut Transaction<'a, Sqlite>, prefix: &str, resource_type: ResourceType,
) -> Result<String, OperationError> {
    loop {
        let id = local_cloud_common::naming::generate_id(prefix, 21)
            .map_err(|err| OperationError::new(ApiErrorKind::ServiceFailure, err.to_string().as_str()))?;

        let mut resource_identifier = ResourceIdentifier::new(&id, resource_type);
        if let Ok(()) = db::resource_identifier::create(tx, &mut resource_identifier).await {
            return Ok(id);
        }
    }
}
