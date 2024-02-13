use aws_sdk_iam::types::Tag;

use crate::http::aws::iam::db::types::tags::DbTag;
use crate::http::aws::iam::types;

pub(crate) fn prepare_for_insert(tags: Option<&[types::tag::Tag]>, parent_id: i64) -> Vec<DbTag> {
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

pub(crate) fn prepare_for_output(tags: &[DbTag]) -> Option<Vec<Tag>> {
    if tags.len() == 0 {
        None
    } else {
        Some(tags.iter().map(|tag| tag.into()).collect())
    }
}
