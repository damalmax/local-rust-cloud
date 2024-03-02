use sqlx::{Sqlite, Transaction};

use local_cloud_axum::local::web::XmlResponse;

use crate::http::aws::iam::operations::error::ActionError;

pub trait Action {
    type Output: ?Sized + Into<XmlResponse>;

    async fn execute<'a>(
        &self, tx: &mut Transaction<'a, Sqlite>, account_id: i64, aws_request_id: &str,
    ) -> Result<Self::Output, ActionError>;
}
