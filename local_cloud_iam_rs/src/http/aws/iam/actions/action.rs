use local_cloud_actix::local::web::XmlResponse;
use local_cloud_db::LocalDb;

use crate::http::aws::iam::actions::error::ApiError;

pub trait Action {
    type Output: ?Sized + Into<XmlResponse>;

    async fn execute(&self, account_id: i64, aws_request_id: &str, db: &LocalDb) -> Result<Self::Output, ApiError>;
}
