use sqlx::FromRow;

use crate::http::aws::iam::db::types::common::Pageable;
use crate::http::aws::iam::types::marker_type::MarkerType;

#[derive(Clone, FromRow, Debug)]
pub(crate) struct DbInlinePolicy {
    pub(crate) id: Option<i64>,
    pub(crate) parent_id: i64,
    pub(crate) policy_name: String,
    pub(crate) policy_document: String,
}

impl DbInlinePolicy {
    pub(crate) fn new(parent_id: i64, policy_name: &str, policy_document: &str) -> Self {
        DbInlinePolicy {
            id: None,
            parent_id,
            policy_name: policy_name.to_owned(),
            policy_document: policy_document.to_owned(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct ListInlinePoliciesQuery {
    pub(crate) parent_id: i64,
    pub(crate) limit: i32,
    pub(crate) skip: i32,
}

impl ListInlinePoliciesQuery {
    pub(crate) fn new(parent_id: i64, max_items: Option<&i32>, marker_type: Option<&MarkerType>) -> Self {
        ListInlinePoliciesQuery {
            parent_id,
            limit: match max_items {
                None => 10,
                Some(v) => *v,
            },

            skip: match marker_type {
                None => 0,
                Some(marker_type) => marker_type.marker().unwrap().truncate_amount,
            },
        }
    }
}

impl Pageable for &ListInlinePoliciesQuery {
    fn limit(&self) -> i32 {
        self.limit
    }

    fn skip(&self) -> i32 {
        self.skip
    }
}
