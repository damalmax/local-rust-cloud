use aws_sdk_iam::operation::attach_group_policy::AttachGroupPolicyOutput;

use local_cloud_db::LocalDb;

use crate::http::aws::iam;
use crate::http::aws::iam::actions::action::Action;
use crate::http::aws::iam::actions::error::ApiError;
use crate::http::aws::iam::actions::wrapper::OutputWrapper;
use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::types::attach_group_policy_request::AttachGroupPolicyRequest;

impl Action for AttachGroupPolicyRequest {
    type Output = OutputWrapper<AttachGroupPolicyOutput>;

    async fn execute(&self, account_id: i64, aws_request_id: &str, db: &LocalDb) -> Result<Self::Output, ApiError> {
        let ctx = OperationCtx::new(account_id, aws_request_id);
        let output = iam::operations::group::attach_group_policy(&ctx, self, db)
            .await
            .map_err(|error| match error {
                OperationError::Service { kind, msg } => ApiError::new(kind, &msg, aws_request_id),
                OperationError::Validation(error) => ApiError::from_validation_error(&error, aws_request_id),
            })?;

        Ok(OutputWrapper::new(output, aws_request_id))
    }
}
