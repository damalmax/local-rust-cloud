use crate::http::aws::iam::types::marker_type::MarkerType;

pub(crate) trait Pageable {
    fn limit(&self) -> i32;
    fn skip(&self) -> i32;
}

#[derive(Debug)]
pub(crate) struct ListByIdQuery {
    pub(crate) parent_id: i64,
    pub(crate) limit: i32,
    pub(crate) skip: i32,
}

impl ListByIdQuery {
    pub(crate) fn new(parent_id: i64, max_items: Option<&i32>, marker_type: Option<&MarkerType>) -> Self {
        ListByIdQuery {
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

impl Pageable for &ListByIdQuery {
    fn limit(&self) -> i32 {
        self.limit
    }

    fn skip(&self) -> i32 {
        self.skip
    }
}
