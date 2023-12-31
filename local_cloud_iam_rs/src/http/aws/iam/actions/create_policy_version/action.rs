use aws_sdk_iam::operation::create_policy_version::CreatePolicyVersionOutput;

use local_cloud_db::LocalDb;

use crate::http::aws::iam;
use crate::http::aws::iam::actions::create_policy_version::LocalCreatePolicyVersion;
use crate::http::aws::iam::actions::error::ApiError;
use crate::http::aws::iam::actions::wrapper::OutputWrapper;
use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;

impl LocalCreatePolicyVersion {
    pub async fn execute(
        &self, account_id: i64, aws_request_id: &str, db: &LocalDb,
    ) -> Result<OutputWrapper<CreatePolicyVersionOutput>, ApiError> {
        let ctx = OperationCtx::new(account_id, aws_request_id);
        let output = iam::operations::policy::create_policy_version(&ctx, self, db)
            .await
            .map_err(|error| match error {
                OperationError::Service { kind, msg } => ApiError::new(kind, &msg, aws_request_id),
                OperationError::Validation(error) => ApiError::from_validation_error(&error, aws_request_id),
            })?;

        Ok(OutputWrapper::new(output, aws_request_id))
    }
}
