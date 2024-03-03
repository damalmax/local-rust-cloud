use aws_sdk_iam::operation::create_policy::CreatePolicyOutput;
use aws_sdk_iam::operation::create_policy_version::CreatePolicyVersionOutput;
use aws_sdk_iam::operation::delete_policy::DeletePolicyOutput;
use aws_sdk_iam::operation::delete_policy_version::DeletePolicyVersionOutput;
use aws_sdk_iam::operation::get_context_keys_for_custom_policy::GetContextKeysForCustomPolicyOutput;
use aws_sdk_iam::operation::get_context_keys_for_principal_policy::GetContextKeysForPrincipalPolicyOutput;
use aws_sdk_iam::operation::get_policy::GetPolicyOutput;
use aws_sdk_iam::operation::get_policy_version::GetPolicyVersionOutput;
use aws_sdk_iam::operation::list_entities_for_policy::ListEntitiesForPolicyOutput;
use aws_sdk_iam::operation::list_policies::ListPoliciesOutput;
use aws_sdk_iam::operation::list_policies_granting_service_access::ListPoliciesGrantingServiceAccessOutput;
use aws_sdk_iam::operation::list_policy_tags::ListPolicyTagsOutput;
use aws_sdk_iam::operation::list_policy_versions::ListPolicyVersionsOutput;
use aws_sdk_iam::operation::set_default_policy_version::SetDefaultPolicyVersionOutput;
use aws_sdk_iam::operation::tag_policy::TagPolicyOutput;
use aws_sdk_iam::operation::untag_policy::UntagPolicyOutput;
use aws_sdk_iam::types::{Policy, PolicyVersion};
use aws_smithy_types::DateTime;
use chrono::Utc;
use sqlx::{Executor, Sqlite, Transaction};

use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::actions::error::ApiErrorKind;
use crate::http::aws::iam::db::types::policy::{
    InsertPolicy, InsertPolicyBuilder, InsertPolicyBuilderError, SelectPolicy,
};
use crate::http::aws::iam::db::types::policy_type::PolicyType;
use crate::http::aws::iam::db::types::policy_version::{
    InsertPolicyVersion, InsertPolicyVersionBuilder, InsertPolicyVersionBuilderError, ListPolicyVersionsQuery,
};
use crate::http::aws::iam::db::types::resource_identifier::ResourceType;
use crate::http::aws::iam::db::types::tags::ListTagsQuery;
use crate::http::aws::iam::operations::common::create_resource_id;
use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::ActionError;
use crate::http::aws::iam::types::create_policy::CreatePolicyRequest;
use crate::http::aws::iam::types::create_policy_version::CreatePolicyVersionRequest;
use crate::http::aws::iam::types::delete_policy::DeletePolicyRequest;
use crate::http::aws::iam::types::delete_policy_version::DeletePolicyVersionRequest;
use crate::http::aws::iam::types::get_context_keys_for_custom_policy::GetContextKeysForCustomPolicyRequest;
use crate::http::aws::iam::types::get_context_keys_for_principal_policy::GetContextKeysForPrincipalPolicyRequest;
use crate::http::aws::iam::types::get_policy::GetPolicyRequest;
use crate::http::aws::iam::types::get_policy_version::GetPolicyVersionRequest;
use crate::http::aws::iam::types::list_entities_for_policy::ListEntitiesForPolicyRequest;
use crate::http::aws::iam::types::list_policies::ListPoliciesRequest;
use crate::http::aws::iam::types::list_policies_granting_service_access::ListPoliciesGrantingServiceAccessRequest;
use crate::http::aws::iam::types::list_policy_tags::ListPolicyTagsRequest;
use crate::http::aws::iam::types::list_policy_versions::ListPolicyVersionsRequest;
use crate::http::aws::iam::types::set_default_policy_version::SetDefaultPolicyVersionRequest;
use crate::http::aws::iam::types::tag_policy::TagPolicyRequest;
use crate::http::aws::iam::types::untag_policy::UntagPolicyRequest;
use crate::http::aws::iam::{constants, db};

pub(crate) async fn find_id_by_arn<'a, E>(executor: E, account_id: i64, arn: &str) -> Result<i64, ActionError>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    match db::policy::find_id_by_arn(executor, account_id, arn).await? {
        Some(policy_id) => Ok(policy_id),
        None => {
            return Err(ActionError::new(
                ApiErrorKind::NoSuchEntity,
                format!("IAM policy with ARN '{}' doesn't exist.", arn).as_str(),
            ));
        }
    }
}

pub(crate) async fn find_by_arn<'a, E>(executor: E, account_id: i64, arn: &str) -> Result<SelectPolicy, ActionError>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    match db::policy::find_by_arn(executor, account_id, arn).await? {
        Some(policy_id) => Ok(policy_id),
        None => {
            return Err(ActionError::new(
                ApiErrorKind::NoSuchEntity,
                format!("IAM policy with ARN '{}' doesn't exist.", arn).as_str(),
            ));
        }
    }
}

pub(crate) async fn create_policy<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &CreatePolicyRequest,
) -> Result<CreatePolicyOutput, ActionError> {
    // validate
    input.validate("$")?;
    let policy_document = input.policy_document().unwrap();

    let policy_id = create_resource_id(tx, constants::policy::PREFIX, ResourceType::Policy).await?;
    let current_time = Utc::now().timestamp();
    let mut insert_policy: InsertPolicy = prepare_policy_for_insert(ctx, input, &policy_id, current_time)
        .map_err(|err| ActionError::new(ApiErrorKind::ServiceFailure, err.to_string().as_str()))?;

    db::policy::create(tx, &mut insert_policy).await?;

    let policy_version_id =
        create_resource_id(tx, constants::policy_version::PREFIX, ResourceType::PolicyVersion).await?;
    let mut policy_version = prepare_policy_version_for_insert(
        ctx,
        policy_document,
        insert_policy.id.unwrap(),
        policy_version_id,
        current_time,
    )
    .map_err(|err| ActionError::new(ApiErrorKind::ServiceFailure, err.to_string().as_str()))?;
    db::policy_version::create(tx, &mut policy_version).await?;

    let mut policy_tags = super::tag::prepare_for_db(input.tags(), insert_policy.id.unwrap());

    db::Tags::Policy.save_all(tx, &mut policy_tags).await?;

    let response_policy_builder = Policy::builder()
        .arn(insert_policy.arn)
        .create_date(DateTime::from_secs(insert_policy.create_date))
        .update_date(DateTime::from_secs(insert_policy.update_date))
        .path(insert_policy.path)
        .policy_id(insert_policy.policy_id)
        .is_attachable(insert_policy.attachable)
        .set_description(insert_policy.description)
        .attachment_count(0)
        .permissions_boundary_usage_count(0)
        .set_tags(super::tag::prepare_for_output(&policy_tags))
        .set_default_version_id(Some(format!("v{}", policy_version.version.unwrap())))
        .policy_name(&insert_policy.policy_name);
    let policy = response_policy_builder.build();
    let output = CreatePolicyOutput::builder().policy(policy).build();
    Ok(output)
}

pub(crate) async fn create_policy_version<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &CreatePolicyVersionRequest,
) -> Result<CreatePolicyVersionOutput, ActionError> {
    // validate
    input.validate("$")?;
    let policy_document = input.policy_document().unwrap();

    let policy_id = db::policy::find_id_by_arn(tx.as_mut(), ctx.account_id, input.policy_arn().unwrap()).await?;
    if policy_id.is_none() {
        return Err(ActionError::new(
            ApiErrorKind::NoSuchEntity,
            format!("Unable to find policy with ARN '{}'.", input.policy_arn().unwrap()).as_str(),
        ));
    }

    let policy_id = policy_id.unwrap();
    check_policy_version_count(tx, policy_id).await?;

    // check whether new policy version should be set as default. True by default
    let set_as_default = input.set_as_default().unwrap_or(true);
    if set_as_default {
        // find and disable previous default policy version
        db::policy_version::disable_default_by_policy_id(tx, policy_id).await?;
    }

    let current_time = Utc::now().timestamp();
    let policy_version = PolicyVersion::builder()
        .is_default_version(set_as_default)
        .create_date(DateTime::from_secs(current_time))
        .version_id("v2")
        .build();

    let policy_version_id =
        create_resource_id(tx, constants::policy_version::PREFIX, ResourceType::PolicyVersion).await?;
    let mut insert_policy_version =
        prepare_policy_version_for_insert(ctx, policy_document, policy_id, policy_version_id, current_time)
            .map_err(|err| ActionError::new(ApiErrorKind::ServiceFailure, err.to_string().as_str()))?;
    db::policy_version::create(tx, &mut insert_policy_version).await?;

    let output = CreatePolicyVersionOutput::builder()
        .policy_version(policy_version)
        .build();
    Ok(output)
}

pub(crate) async fn list_policy_versions<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &ListPolicyVersionsRequest,
) -> Result<ListPolicyVersionsOutput, ActionError> {
    input.validate("$")?;

    let policy_arn = input.policy_arn().unwrap().trim();

    let policy_id = find_id_by_arn(tx.as_mut(), ctx.account_id, policy_arn).await?;

    let query = ListPolicyVersionsQuery {
        policy_id,
        limit: match input.max_items() {
            None => 10,
            Some(v) => *v,
        },
        skip: match input.marker_type() {
            None => 0,
            Some(marker_type) => marker_type.marker().unwrap().truncate_amount,
        },
    };

    let found_policy_versions = db::policy_version::find_by_policy_id(tx.as_mut(), &query).await?;

    let policy_versions = super::common::convert_and_limit(&found_policy_versions, query.limit);
    let marker = super::common::create_encoded_marker(&query, found_policy_versions.len())?;

    let output = ListPolicyVersionsOutput::builder()
        .set_versions(policy_versions)
        .set_is_truncated(marker.as_ref().map(|_v| true))
        .set_marker(marker)
        .build();
    Ok(output)
}

async fn check_policy_version_count<'a>(tx: &mut Transaction<'a, Sqlite>, policy_id: i64) -> Result<(), ActionError> {
    let policy_version_count = db::policy_version::count_by_policy_id(tx, policy_id).await?;
    if policy_version_count >= constants::policy_version::POLICY_VERSION_MAX_COUNT {
        return Err(ActionError::new(
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

pub(crate) async fn list_policies<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &ListPoliciesRequest,
) -> Result<ListPoliciesOutput, ActionError> {
    input.validate("$")?;

    let query = input.into();

    let found_policies: Vec<SelectPolicy> = db::policy::list(tx.as_mut(), ctx.account_id, &query).await?;

    let policies = super::common::convert_and_limit(&found_policies, query.limit);
    let marker = super::common::create_encoded_marker(&query, found_policies.len())?;

    let output = ListPoliciesOutput::builder()
        .set_policies(policies)
        .set_is_truncated(marker.as_ref().map(|_v| true))
        .set_marker(marker)
        .build();
    Ok(output)
}

pub(crate) async fn list_policy_tags<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &ListPolicyTagsRequest,
) -> Result<ListPolicyTagsOutput, ActionError> {
    input.validate("$")?;

    let found_policy_id = find_id_by_arn(tx.as_mut(), ctx.account_id, input.policy_arn().unwrap().trim()).await?;

    let query = ListTagsQuery::new(input.max_items(), input.marker_type());

    let found_tags = db::Tags::Policy.list(tx.as_mut(), found_policy_id, &query).await?;
    let tags = super::common::convert_and_limit(&found_tags, query.limit);
    let marker = super::common::create_encoded_marker(&query, found_tags.len())?;

    let output = ListPolicyTagsOutput::builder()
        .set_tags(tags)
        .set_is_truncated(marker.as_ref().map(|_v| true))
        .set_marker(marker)
        .build()
        .unwrap();
    Ok(output)
}

pub(crate) async fn tag_policy<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &TagPolicyRequest,
) -> Result<TagPolicyOutput, ActionError> {
    input.validate("$")?;

    let policy_id = find_id_by_arn(tx.as_mut(), ctx.account_id, input.policy_arn().unwrap().trim()).await?;
    let mut policy_tags = super::tag::prepare_for_db(input.tags(), policy_id);

    db::Tags::Policy.save_all(tx, &mut policy_tags).await?;
    let count = db::Tags::Policy.count(tx.as_mut(), policy_id).await?;
    if count > constants::tag::MAX_COUNT {
        return Err(ActionError::new(
            ApiErrorKind::LimitExceeded,
            format!("Cannot assign more than {} tags to IAM policy.", constants::tag::MAX_COUNT).as_str(),
        ));
    }

    let output = TagPolicyOutput::builder().build();
    Ok(output)
}

fn prepare_policy_for_insert(
    ctx: &OperationCtx, policy_input: &CreatePolicyRequest, policy_id: &str, current_time: i64,
) -> Result<InsertPolicy, InsertPolicyBuilderError> {
    let path = policy_input.path().unwrap_or("/");
    let policy_name = policy_input.policy_name().unwrap().trim();
    let arn = format!("arn:aws:iam::{:0>12}:policy{}{}", ctx.account_id, path, policy_name);
    InsertPolicyBuilder::default()
        // The property will be initialized during insert
        .id(None)
        .account_id(ctx.account_id)
        .path(path.to_owned())
        .description(policy_input.description().map(|value| value.to_owned()))
        .arn(arn)
        .policy_id(policy_id.to_owned())
        .policy_name(policy_name.to_owned())
        // 'IsAttachable' should be 'true' by default
        .attachable(policy_input.is_attachable().unwrap_or(true))
        .policy_type(PolicyType::CustomerManaged)
        .create_date(current_time)
        .update_date(current_time)
        .build()
}

fn prepare_policy_version_for_insert(
    ctx: &OperationCtx, policy_document: &str, policy_id: i64, policy_version_id: String, current_time: i64,
) -> Result<InsertPolicyVersion, InsertPolicyVersionBuilderError> {
    InsertPolicyVersionBuilder::default()
        .id(None)
        .is_default(true)
        .policy_id(policy_id)
        .policy_document(policy_document.to_owned())
        .policy_version_id(policy_version_id)
        .version(None)
        .account_id(ctx.account_id)
        .create_date(current_time)
        .build()
}

pub(crate) async fn untag_policy<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &UntagPolicyRequest,
) -> Result<UntagPolicyOutput, ActionError> {
    input.validate("$")?;

    let policy_id = find_id_by_arn(tx.as_mut(), ctx.account_id, input.policy_arn().unwrap().trim()).await?;

    db::Tags::Policy.delete_all(tx, policy_id, &input.tag_keys()).await?;

    let output = UntagPolicyOutput::builder().build();
    Ok(output)
}

pub(crate) async fn get_policy<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &GetPolicyRequest,
) -> Result<GetPolicyOutput, ActionError> {
    input.validate("$")?;

    let arn = input.policy_arn().unwrap();
    let select_policy = find_by_arn(tx.as_mut(), ctx.account_id, arn).await?;

    let policy = Policy::builder()
        .arn(&select_policy.arn)
        .create_date(DateTime::from_secs(select_policy.create_date))
        .update_date(DateTime::from_secs(select_policy.update_date))
        .path(&select_policy.path)
        .policy_id(select_policy.policy_id)
        .is_attachable(select_policy.is_attachable)
        .set_description(select_policy.description)
        .attachment_count(select_policy.attachment_count)
        .permissions_boundary_usage_count(select_policy.permissions_boundary_usage_count)
        .set_default_version_id(Some(format!("v{}", select_policy.version)))
        .policy_name(&select_policy.policy_name)
        .build();
    let output = GetPolicyOutput::builder().policy(policy).build();
    Ok(output)
}

pub(crate) async fn get_policy_version<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &GetPolicyVersionRequest,
) -> Result<GetPolicyVersionOutput, ActionError> {
    input.validate("$")?;

    let output = GetPolicyVersionOutput::builder().build();
    Ok(output)
}

pub(crate) async fn set_default_policy_version<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &SetDefaultPolicyVersionRequest,
) -> Result<SetDefaultPolicyVersionOutput, ActionError> {
    input.validate("$")?;

    let output = SetDefaultPolicyVersionOutput::builder().build();
    Ok(output)
}

pub(crate) async fn delete_policy<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &DeletePolicyRequest,
) -> Result<DeletePolicyOutput, ActionError> {
    input.validate("$")?;

    let output = DeletePolicyOutput::builder().build();
    Ok(output)
}

pub(crate) async fn delete_policy_version<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &DeletePolicyVersionRequest,
) -> Result<DeletePolicyVersionOutput, ActionError> {
    input.validate("$")?;

    let output = DeletePolicyVersionOutput::builder().build();
    Ok(output)
}

pub(crate) async fn get_context_keys_for_custom_policy<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &GetContextKeysForCustomPolicyRequest,
) -> Result<GetContextKeysForCustomPolicyOutput, ActionError> {
    input.validate("$")?;

    let output = GetContextKeysForCustomPolicyOutput::builder().build();
    Ok(output)
}

pub(crate) async fn get_context_keys_for_principal_policy<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &GetContextKeysForPrincipalPolicyRequest,
) -> Result<GetContextKeysForPrincipalPolicyOutput, ActionError> {
    input.validate("$")?;

    let output = GetContextKeysForPrincipalPolicyOutput::builder().build();
    Ok(output)
}

pub(crate) async fn list_entities_for_policy<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &ListEntitiesForPolicyRequest,
) -> Result<ListEntitiesForPolicyOutput, ActionError> {
    input.validate("$")?;

    let output = ListEntitiesForPolicyOutput::builder().build();
    Ok(output)
}

pub(crate) async fn list_policies_granting_service_access<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &ListPoliciesGrantingServiceAccessRequest,
) -> Result<ListPoliciesGrantingServiceAccessOutput, ActionError> {
    input.validate("$")?;

    let output = ListPoliciesGrantingServiceAccessOutput::builder().build().unwrap();
    Ok(output)
}
