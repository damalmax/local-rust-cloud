use chrono::prelude::Utc;

use aws_sdk_iam::{
    operation::create_policy::CreatePolicyOutput,
    types::{Policy as IamPolicy, Tag},
};
use local_rust_cloud_sqlite::Database;

use crate::{
    repository::{policy::PolicyRepo, policy_tag::PolicyTagRepo},
    types::policy::Policy,
};

use super::{
    action::Iam, create_policy_request::LocalCreatePolicyInput, create_policy_response::LocalCreatePolicyOutput, errors::IamApiError,
    validators::create_policy::validate, OutputWrapper,
};

impl Iam {
    pub async fn create_policy<I: Into<LocalCreatePolicyInput>>(
        db: &Database, account_id: i64, request_id: &str, input: I,
    ) -> Result<LocalCreatePolicyOutput, IamApiError> {
        let input: LocalCreatePolicyInput = input.into();
        // validate request
        validate(&request_id, &input)?;

        let mut tx = db
            .new_tx()
            .await
            .map_err(|_| IamApiError::internal_server_error(request_id, "Failed to BEGIN transaction"))?;
        let policy_repo = PolicyRepo::new();
        // let tag_repo = PolicyTagRepo::new();

        let arn = format!("arn:aws:iam:{:0>12}:policy/{}", account_id, input.policy_name().unwrap());
        let current_time = Utc::now().timestamp();
        let mut policy = Policy::builder()
            .from_policy_input(&input)
            .account_id(account_id)
            .arn(arn)
            .is_attachable(true)
            .create_date(current_time)
            .update_date(current_time)
            .build()?;

        policy_repo.save(&mut tx, &mut policy).await.expect("failed to save policy");
        let mut tags = vec![];
        if input.tags().is_some() {
            for tag in input.tags().unwrap() {
                let tag = Tag::builder()
                    .key(tag.key().map(|v| v.to_string()).unwrap_or(String::new()))
                    .value(tag.value().map(|v| v.to_string()).unwrap_or(String::new()))
                    .build();
                tags.push(tag);
            }
        }

        // tag_repo.save_all(&mut tx, tags).await.expect("failed to save policy tags");

        let response_policy_builder = IamPolicy::builder().policy_name(input.policy_name().unwrap());
        let policy = response_policy_builder.set_tags(Option::Some(tags)).build();
        let result = CreatePolicyOutput::builder().policy(policy).build();

        tx.commit()
            .await
            .map_err(|_| IamApiError::internal_server_error(request_id, "failed to COMMIT transaction"))?;
        Result::Ok(OutputWrapper::new(result, request_id))
    }
}
