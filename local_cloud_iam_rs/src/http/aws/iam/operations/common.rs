use sqlx::{Sqlite, Transaction};

use crate::http::aws::iam::actions::error::ApiErrorKind;
use crate::http::aws::iam::db;
use crate::http::aws::iam::db::types::common::Pageable;
use crate::http::aws::iam::db::types::resource_identifier::{ResourceIdentifier, ResourceType};
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::types::marker_type::Marker;

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

pub(crate) fn create_encoded_marker(
    pageable: impl Pageable, found_items: usize,
) -> Result<Option<String>, OperationError> {
    if pageable.limit() < found_items as i32 {
        let marker = Marker::new(pageable.limit() + pageable.skip())
            .encode()
            .map_err(|_err| OperationError::new(ApiErrorKind::ServiceFailure, "Failed to generate Marker value."))?;
        Ok(Some(marker))
    } else {
        Ok(None)
    }
}

pub(crate) fn convert_and_limit<S: Sized, T: Sized + for<'a> From<&'a S>>(
    source_slice: &[S], limit: i32,
) -> Option<Vec<T>> {
    if source_slice.len() == 0 {
        Some(vec![])
    } else {
        let mut result: Vec<T> = vec![];
        for i in 0..limit {
            let item = source_slice.get(i as usize);
            match item {
                None => break,
                Some(item) => {
                    result.push(item.into());
                }
            }
        }
        Some(result)
    }
}
