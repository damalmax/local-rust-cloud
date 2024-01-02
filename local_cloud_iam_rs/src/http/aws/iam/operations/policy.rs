use aws_sdk_iam::operation::create_policy::CreatePolicyOutput;
use aws_sdk_iam::types::{Policy, Tag};
use chrono::Utc;

use local_cloud_db::LocalDb;

use crate::http::aws::iam::actions::create_policy::LocalCreatePolicy;
use crate::http::aws::iam::actions::error::ApiErrorKind;
use crate::http::aws::iam::db::types::policy::{InsertPolicy, InsertPolicyBuilder, InsertPolicyBuilderError};
use crate::http::aws::iam::db::types::policy_type::PolicyType;
use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::validate;
use crate::http::aws::iam::validate::IamValidator;
use crate::http::aws::iam::{constants, db};

pub async fn create_policy(
    ctx: &OperationCtx, policy_input: &LocalCreatePolicy, db: &LocalDb,
) -> Result<CreatePolicyOutput, OperationError> {
    validate::create_policy::validate(policy_input)?;
    // The presence of the policy document is already validated with general validate call above
    let policy_document =
        validate::policy_document::validate_and_minify_managed(policy_input.policy_document().unwrap())?;

    // TODO: all IDs should be unique across account
    let policy_id = local_cloud_common::naming::generate_id(constants::policy::MANAGED_POLICY_PREFIX, 21)
        .map_err(|err| OperationError::new(ApiErrorKind::ServiceFailure, err.to_string().as_str()))?;

    let mut policy: InsertPolicy = prepare_policy_for_insert(ctx, policy_input, &policy_id)
        .map_err(|err| OperationError::new(ApiErrorKind::ServiceFailure, err.to_string().as_str()))?;

    let mut tx = db.new_tx().await?;
    db::policy::save(&mut tx, &mut policy).await?;
    let mut tags = vec![];

    let input_tags = policy_input.tags();
    if input_tags.is_some() {
        for local_tag in input_tags.unwrap() {
            let tag = Tag::builder()
                .key(local_tag.key().unwrap())
                .value(local_tag.value().unwrap())
                .build()
                .unwrap();
            tags.push(tag);
        }
    }
    // db::policy_tag::save_all(&mut tx)
    //     .as_mut()
    //     .expect("failed to save policy tags");

    let response_policy_builder = Policy::builder().set_tags(Some(tags)).policy_name(&policy.policy_name);
    let policy = response_policy_builder.build();
    let output = CreatePolicyOutput::builder().policy(policy).build();

    tx.commit().await?;

    Ok(output)
}

fn prepare_policy_for_insert(
    ctx: &OperationCtx, policy_input: &LocalCreatePolicy, policy_id: &str,
) -> Result<InsertPolicy, InsertPolicyBuilderError> {
    let current_time = Utc::now().timestamp();
    let policy_name = policy_input.policy_name().unwrap().trim();
    let arn = format!("arn:aws:iam:{:0>12}:policy/{}", ctx.account_id, policy_name);
    InsertPolicyBuilder::default()
        // The property will be initialized during insert
        .id(None)
        .account_id(ctx.account_id)
        .path(policy_input.path().unwrap_or("/").to_owned())
        .description(policy_input.description().map(|value| value.to_owned()))
        .arn(arn)
        .policy_id(policy_id.to_owned())
        .policy_name(policy_name.to_owned())
        // 'IsAttachable' should be 'true' by default
        .attachable(policy_input.attachable().unwrap_or(true))
        .policy_type(PolicyType::CustomerManaged)
        .create_date(current_time)
        .update_date(current_time)
        .build()
}
