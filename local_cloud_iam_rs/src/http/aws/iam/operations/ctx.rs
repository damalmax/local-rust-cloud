use axum::async_trait;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use local_cloud_axum::local::web::AwsRequestHeaders;
use uuid::Uuid;

use local_cloud_db::LocalDb;

#[derive(Debug)]
#[allow(dead_code)]
pub struct OperationCtx {
    pub(crate) account_id: i64,
    pub(crate) user_id: i64,
    pub(crate) aws_request_id: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for OperationCtx
where
    S: Send + Sync,
    LocalDb: FromRef<S>,
{
    type Rejection = ();

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let db = LocalDb::from_ref(state);
        let mut tx = db.new_tx().await.expect("Failed to acquire connection");

        for (name, value) in &parts.headers {
            println!("{name}: {}", value.to_str().unwrap_or_default());
        }
        //
        let amz_date = parts.headers.get(AwsRequestHeaders::AmzDate.as_str()).unwrap();
        // let amz_security_token = parts
        //     .headers
        //     .get("x-amz-security-token")
        //     .expect("Security token is not provided");

        let aws_sdk_invocation_id = parts
            .headers
            .get("amz-sdk-invocation-id")
            .map(|h| h.to_str().unwrap_or_default().to_string())
            .unwrap_or(Uuid::new_v4().to_string());

        Ok(OperationCtx {
            account_id: 1,
            user_id: 1,
            aws_request_id: aws_sdk_invocation_id,
        })
    }
}
