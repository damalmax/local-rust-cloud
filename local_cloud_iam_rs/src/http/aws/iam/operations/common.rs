use aws_sdk_iam::types::Tag;
use sqlx::{Sqlite, Transaction};

use crate::http::aws::iam::actions::error::ApiErrorKind;
use crate::http::aws::iam::db::types::resource_identifier::{ResourceIdentifier, ResourceType};
use crate::http::aws::iam::db::types::tag::DbTag;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::{db, types};

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

pub(crate) fn prepare_tags_for_insert(tags: Option<&[types::tag::Tag]>, parent_id: i64) -> Vec<DbTag> {
    match tags {
        None => vec![],
        Some(tags) => {
            let mut result = vec![];
            for tag in tags {
                let db_tag = DbTag::new(parent_id, tag.key().unwrap(), tag.value().unwrap_or(""));
                result.push(db_tag);
            }
            result
        }
    }
}

pub(crate) fn prepare_tags_for_output(tags: Vec<DbTag>) -> Option<Vec<Tag>> {
    if tags.len() == 0 {
        None
    } else {
        Some(tags.iter().map(|tag| tag.into()).collect())
    }
}
