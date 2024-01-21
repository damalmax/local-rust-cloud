use aws_sdk_iam::operation::create_policy::CreatePolicyOutput;
use aws_sdk_iam::operation::create_policy_version::CreatePolicyVersionOutput;
use aws_sdk_iam::operation::list_policies::ListPoliciesOutput;
use aws_sdk_iam::types::{Policy, PolicyVersion, Tag};
use aws_smithy_types::DateTime;
use chrono::Utc;
use sqlx::{Sqlite, Transaction};

use local_cloud_db::LocalDb;
use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::actions::error::ApiErrorKind;
use crate::http::aws::iam::db::types::policy::{InsertPolicy, InsertPolicyBuilder, InsertPolicyBuilderError};
use crate::http::aws::iam::db::types::policy_tag::{DbPolicyTag, DbPolicyTagBuilder};
use crate::http::aws::iam::db::types::policy_type::PolicyType;
use crate::http::aws::iam::db::types::policy_version::{
    InsertPolicyVersion, InsertPolicyVersionBuilder, InsertPolicyVersionBuilderError,
};
use crate::http::aws::iam::db::types::resource_identifier::{ResourceIdentifier, ResourceType};
use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::types::create_policy_request::CreatePolicyRequest;
use crate::http::aws::iam::types::create_policy_version_request::CreatePolicyVersionRequest;
use crate::http::aws::iam::types::list_policies_request::ListPoliciesRequest;
use crate::http::aws::iam::validate;
use crate::http::aws::iam::{constants, db, types};

pub async fn create_policy(
    ctx: &OperationCtx, input: &CreatePolicyRequest, db: &LocalDb,
) -> Result<CreatePolicyOutput, OperationError> {
    // validate
    input.validate("$")?;
    let policy_document = validate::policy_document::validate_and_minify_managed(input.policy_document())?;

    // init transaction
    let mut tx = db.new_tx().await?;

    let policy_id = create_resource_id(&mut tx, constants::policy::MANAGED_POLICY_PREFIX, ResourceType::Policy).await?;
    let current_time = Utc::now().timestamp();
    let mut policy: InsertPolicy = prepare_policy_for_insert(ctx, input, &policy_id, current_time)
        .map_err(|err| OperationError::new(ApiErrorKind::ServiceFailure, err.to_string().as_str()))?;

    db::policy::create(&mut tx, &mut policy).await?;

    let policy_version_id =
        create_resource_id(&mut tx, constants::policy_version::POLICY_VERSION_PREFIX, ResourceType::PolicyVersion)
            .await?;
    let mut policy_version =
        prepare_policy_version_for_insert(ctx, policy_document, policy.id.unwrap(), policy_version_id, current_time)
            .map_err(|err| OperationError::new(ApiErrorKind::ServiceFailure, err.to_string().as_str()))?;
    db::policy_version::create(&mut tx, &mut policy_version).await?;

    let mut policy_tags = prepare_tags_for_insert(input.tags(), policy.id.unwrap());

    db::policy_tag::save_all(&mut tx, &mut policy_tags).await?;

    let response_policy_builder = Policy::builder()
        .arn(policy.arn)
        .create_date(DateTime::from_secs(policy.create_date))
        .update_date(DateTime::from_secs(policy.update_date))
        .path(policy.path)
        .policy_id(policy.policy_id)
        .is_attachable(policy.attachable)
        .set_description(policy.description)
        .attachment_count(0)
        .permissions_boundary_usage_count(0)
        .set_tags(prepare_tags_for_output(policy_tags))
        .set_default_version_id(Some(format!("v{}", policy_version.version.unwrap())))
        .policy_name(&policy.policy_name);
    let policy = response_policy_builder.build();
    let output = CreatePolicyOutput::builder().policy(policy).build();

    tx.commit().await?;

    Ok(output)
}

pub async fn create_policy_version(
    ctx: &OperationCtx, input: &CreatePolicyVersionRequest, db: &LocalDb,
) -> Result<CreatePolicyVersionOutput, OperationError> {
    // validate
    input.validate("$")?;
    let policy_document = validate::policy_document::validate_and_minify_managed(input.policy_document())?;

    // init transaction
    let mut tx = db.new_tx().await?;

    let policy_id = db::policy::find_id_by_arn(&mut tx, input.policy_arn().unwrap()).await?;
    if policy_id.is_none() {
        return Err(OperationError::new(
            ApiErrorKind::NoSuchEntity,
            format!("Unable to find policy with ARN '{}'.", input.policy_arn().unwrap()).as_str(),
        ));
    }

    let policy_id = policy_id.unwrap();
    check_policy_version_count(&mut tx, policy_id).await?;

    // check whether new policy version should be set as default. True by default
    let set_as_default = input.set_as_default().map(|v| v.as_bool()).unwrap_or(true);
    if set_as_default {
        // find and disable previous default policy version
        db::policy_version::disable_default_by_policy_id(&mut tx, policy_id).await?;
    }

    let current_time = Utc::now().timestamp();
    let policy_version = PolicyVersion::builder()
        .is_default_version(set_as_default)
        .create_date(DateTime::from_secs(current_time))
        .version_id("v2")
        .build();

    let policy_version_id =
        create_resource_id(&mut tx, constants::policy_version::POLICY_VERSION_PREFIX, ResourceType::PolicyVersion)
            .await?;
    let mut insert_policy_version =
        prepare_policy_version_for_insert(ctx, policy_document, policy_id, policy_version_id, current_time)
            .map_err(|err| OperationError::new(ApiErrorKind::ServiceFailure, err.to_string().as_str()))?;
    db::policy_version::create(&mut tx, &mut insert_policy_version).await?;

    let output = CreatePolicyVersionOutput::builder()
        .policy_version(policy_version)
        .build();
    tx.commit().await?;

    Ok(output)
}

async fn check_policy_version_count<'a>(
    tx: &mut Transaction<'a, Sqlite>, policy_id: i64,
) -> Result<(), OperationError> {
    let policy_version_count = db::policy_version::count_by_policy_id(tx, policy_id).await?;
    if policy_version_count >= constants::policy_version::POLICY_VERSION_MAX_COUNT {
        return Err(OperationError::new(
            ApiErrorKind::LimitExceeded,
            format!(
                "Number of Policy Versions cannot be greater than '{}'. Actual count: '{}'.",
                constants::policy_version::POLICY_VERSION_MAX_COUNT,
                policy_version_count
            )
            .as_str(),
        ));
    }
    Ok(())
}

pub async fn list_policies(
    _ctx: &OperationCtx, _list_policies_input: &ListPoliciesRequest, _db: &LocalDb,
) -> Result<ListPoliciesOutput, OperationError> {
    let output = ListPoliciesOutput::builder().build();

    Ok(output)
}

async fn create_resource_id<'a>(
    tx: &mut Transaction<'a, Sqlite>, prefix: &str, resource_type: ResourceType,
) -> Result<String, OperationError> {
    loop {
        let id = local_cloud_common::naming::generate_id(prefix, 21)
            .map_err(|err| OperationError::new(ApiErrorKind::ServiceFailure, err.to_string().as_str()))?;

        let mut resource_identifier = ResourceIdentifier::new(&id, resource_type);
        if let Ok(()) = db::resource_identifier::create(tx, &mut resource_identifier).await {
            return Ok(id);
        }
    }
}

fn prepare_policy_for_insert(
    ctx: &OperationCtx, policy_input: &CreatePolicyRequest, policy_id: &str, current_time: i64,
) -> Result<InsertPolicy, InsertPolicyBuilderError> {
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
        .attachable(policy_input.is_attachable().map(|a| a.as_bool()).unwrap_or(true))
        .policy_type(PolicyType::CustomerManaged)
        .create_date(current_time)
        .update_date(current_time)
        .build()
}

fn prepare_policy_version_for_insert(
    ctx: &OperationCtx, policy_document: String, policy_id: i64, policy_version_id: String, current_time: i64,
) -> Result<InsertPolicyVersion, InsertPolicyVersionBuilderError> {
    InsertPolicyVersionBuilder::default()
        .id(None)
        .is_default(true)
        .policy_id(policy_id)
        .policy_document(policy_document)
        .policy_version_id(policy_version_id)
        .version(None)
        .account_id(ctx.account_id)
        .create_date(current_time)
        .build()
}

fn prepare_tags_for_insert(tags: Option<&[types::tag::Tag]>, policy_id: i64) -> Vec<DbPolicyTag> {
    match tags {
        None => vec![],
        Some(tags) => {
            let mut policy_tags = vec![];
            for tag in tags {
                let policy_tag = DbPolicyTagBuilder::default()
                    .id(None)
                    .key(tag.key().unwrap().to_owned())
                    .value(tag.value().unwrap().to_owned())
                    .policy_id(policy_id)
                    .build()
                    .unwrap();
                policy_tags.push(policy_tag);
            }
            policy_tags
        }
    }
}

fn prepare_tags_for_output(tags: Vec<DbPolicyTag>) -> Option<Vec<Tag>> {
    if tags.len() == 0 {
        None
    } else {
        Some(
            tags.iter()
                .map(|tag| Tag::builder().key(&tag.key).value(&tag.value).build().unwrap())
                .collect(),
        )
    }
}
