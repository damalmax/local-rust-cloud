use aws_sdk_iam::operation::create_policy::CreatePolicyOutput;
use aws_sdk_iam::types::{Policy, Tag};
use aws_smithy_types::error::ErrorMetadata;
use chrono::Utc;
use futures::executor::block_on;
use log::error;

use local_cloud_db::Database;

use crate::http::aws::iam::actions::create_policy::LocalCreatePolicy;
use crate::http::aws::iam::actions::error::IamError;
use crate::http::aws::iam::actions::error::IamErrorKind::ServiceFailure;
use crate::http::aws::iam::actions::validate::IamValidator;
use crate::http::aws::iam::actions::wrapper::OutputWrapper;
use crate::http::aws::iam::repository;
use crate::http::aws::iam::types::policy::DbPolicy;

impl LocalCreatePolicy {
    pub fn execute(&self, account_id: i64, db: &Database) -> Result<OutputWrapper<CreatePolicyOutput>, IamError> {
        self.validate()?;
        let policy_name = self.policy_name().unwrap_or("").trim();
        let mut tx = db.new_tx().map_err(|err| {
            error!("Failed to BEGIN transaction. Error: {err}");
            IamError::new(ServiceFailure, "Failed to BEGIN transaction", self.aws_request_id())
        })?;

        let arn = format!("arn:aws:iam:{:0>12}:policy/{}", account_id, policy_name);
        let current_time = Utc::now().timestamp();
        let mut policy = DbPolicy::builder()
            .path(self.path().unwrap_or(""))
            .policy_document(self.policy_document().unwrap_or(""))
            .description(self.description().unwrap_or(""))
            .account_id(account_id)
            .arn(arn)
            .is_attachable(true)
            .create_date(current_time)
            .update_date(current_time)
            .build()?;

        repository::policy::save(&mut tx, &mut policy).expect("failed to save policy");
        let mut tags = vec![];

        let input_tags = self.tags();
        if input_tags.is_some() {
            for local_tag in self.tags().unwrap() {
                let tag = Tag::builder()
                    .key(local_tag.key().unwrap())
                    .value(local_tag.value().unwrap())
                    .build()
                    .unwrap();
                tags.push(tag);
            }
        }
        // repository::policy_tag::save_all(&mut tx, )
        //     .expect("failed to save policy tags");

        let response_policy_builder = Policy::builder().set_tags(Some(tags)).policy_name(policy_name);
        let policy = response_policy_builder.build();
        let result = CreatePolicyOutput::builder().policy(policy).build();

        block_on(async { tx.commit().await }).map_err(|err| {
            error!("Failed to commit transaction. Error: {err}");
            IamError::new(ServiceFailure, "Failed to COMMIT transaction", self.aws_request_id())
        })?;
        Ok(OutputWrapper::new(result, self.aws_request_id()))
    }

    pub(crate) fn error_metadata(&self) -> ErrorMetadata {
        ErrorMetadata::builder()
            .custom("aws_request_id", self.aws_request_id())
            .build()
    }
}
