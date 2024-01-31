use aws_sdk_iam::operation::create_group::CreateGroupOutput;

use local_cloud_db::LocalDb;

use crate::http::aws::iam;
use crate::http::aws::iam::actions::action::Action;
use crate::http::aws::iam::actions::error::{ApiError, ApiErrorKind};
use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;
use crate::http::aws::iam::types::create_group_request::CreateGroupRequest;

impl Action for CreateGroupRequest {
    type Output = OutputWrapper<CreateGroupOutput>;

    async fn execute(&self, account_id: i64, aws_request_id: &str, db: &LocalDb) -> Result<Self::Output, ApiError> {
        let ctx = OperationCtx::new(account_id, aws_request_id);
        let output = iam::operations::group::create_group(&ctx, self, db)
            .await
            .map_err(|error| match error {
                OperationError::Service { kind, msg } => {
                    if kind == ApiErrorKind::EntityAlreadyExists {
                        ApiError::new(
                            kind,
                            format!("IAM group with name '{}' already exists.", self.group_name().unwrap().trim()),
                            aws_request_id,
                        )
                    } else {
                        ApiError::new(kind, &msg, aws_request_id)
                    }
                }
                OperationError::Validation(error) => ApiError::from_validation_error(&error, aws_request_id),
            })?;

        Ok(OutputWrapper::new(output, aws_request_id))
    }
}
