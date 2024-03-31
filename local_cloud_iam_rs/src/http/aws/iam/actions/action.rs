use sqlx::{Sqlite, Transaction};

use local_cloud_axum::local::web::XmlResponse;

use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::ActionError;

pub trait Action {
    type Output: ?Sized + Into<XmlResponse>;

    async fn execute<'a>(
        &self, tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx,
    ) -> Result<Self::Output, ActionError>;
}
