use std::time::SystemTime;

use aws_sdk_iam::operation::create_user::CreateUserOutput;
use aws_sdk_iam::types::User;
use aws_smithy_types::DateTime;

use local_cloud_db::LocalDb;

use crate::http::aws::iam::actions::create_user::LocalCreateUser;
use crate::http::aws::iam::actions::error::ApiError;
use crate::http::aws::iam::actions::wrapper::OutputWrapper;

impl LocalCreateUser {
    pub async fn execute(
        &self, _account_id: i64, aws_request_id: &str, _db: &LocalDb,
    ) -> Result<OutputWrapper<CreateUserOutput>, ApiError> {
        let user = User::builder()
            .path("/")
            .arn("")
            .user_name("name")
            .user_id("id")
            .create_date(DateTime::from(SystemTime::now()))
            .build()
            .unwrap();
        let result = CreateUserOutput::builder().user(user).build();
        Ok(OutputWrapper::new(result, aws_request_id))
    }
}
