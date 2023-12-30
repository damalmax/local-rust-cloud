use aws_sdk_iam::operation::create_policy::CreatePolicyOutput;
use aws_sdk_iam::types::{Policy, Tag};
use chrono::Utc;
use log::error;

use local_cloud_db::LocalDb;

use crate::http::aws::iam::actions::create_policy::LocalCreatePolicy;
use crate::http::aws::iam::actions::error::IamError;
use crate::http::aws::iam::actions::error::IamErrorKind::ServiceFailure;
use crate::http::aws::iam::actions::validate::IamValidator;
use crate::http::aws::iam::actions::wrapper::OutputWrapper;
use crate::http::aws::iam::repository;
use crate::http::aws::iam::types::policy::DbPolicy;

impl LocalCreatePolicy {
    pub async fn execute(
        &self, account_id: i64, aws_request_id: &str, db: &LocalDb,
    ) -> Result<OutputWrapper<CreatePolicyOutput>, IamError> {
        self.validate(aws_request_id)?;
        let policy_name = self.policy_name().unwrap_or("").trim();
        let mut tx = db.new_tx().await.map_err(|err| {
            error!("Failed to BEGIN transaction. Error: {err}");
            IamError::new(ServiceFailure, "Failed to BEGIN transaction", aws_request_id)
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

        repository::policy::save(&mut tx, &mut policy)
            .await
            .expect("failed to save policy");
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

        tx.commit().await.map_err(|err| {
            error!("Failed to commit transaction. Error: {err}");
            IamError::new(ServiceFailure, "Failed to COMMIT transaction", aws_request_id)
        })?;
        Ok(OutputWrapper::new(result, aws_request_id))
    }
}
