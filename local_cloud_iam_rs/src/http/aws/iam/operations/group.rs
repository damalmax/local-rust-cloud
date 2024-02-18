use aws_sdk_iam::operation::add_user_to_group::AddUserToGroupOutput;
use aws_sdk_iam::operation::attach_group_policy::AttachGroupPolicyOutput;
use aws_sdk_iam::operation::create_group::CreateGroupOutput;
use aws_sdk_iam::operation::get_group::GetGroupOutput;
use aws_sdk_iam::operation::get_group_policy::GetGroupPolicyOutput;
use aws_sdk_iam::operation::list_group_policies::ListGroupPoliciesOutput;
use aws_sdk_iam::operation::list_groups::ListGroupsOutput;
use aws_sdk_iam::operation::list_groups_for_user::ListGroupsForUserOutput;
use aws_sdk_iam::operation::put_group_policy::PutGroupPolicyOutput;
use aws_sdk_iam::types::Group;
use aws_smithy_types::DateTime;
use chrono::Utc;
use sqlx::{Executor, Sqlite};

use local_cloud_db::LocalDb;
use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::actions::error::ApiErrorKind;
use crate::http::aws::iam::db::types::group::{
    InsertGroup, InsertGroupBuilder, InsertGroupBuilderError, ListGroupsByUserQuery, ListGroupsQuery, SelectGroup,
};
use crate::http::aws::iam::db::types::inline_policy::{DbInlinePolicy, ListInlinePoliciesQuery};
use crate::http::aws::iam::db::types::resource_identifier::ResourceType;
use crate::http::aws::iam::db::types::user::ListUsersByGroupQuery;
use crate::http::aws::iam::operations::common::create_resource_id;
use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::types::add_user_to_group_request::AddUserToGroupRequest;
use crate::http::aws::iam::types::attach_group_policy_request::AttachGroupPolicyRequest;
use crate::http::aws::iam::types::create_group_request::CreateGroupRequest;
use crate::http::aws::iam::types::get_group_policy_request::GetGroupPolicyRequest;
use crate::http::aws::iam::types::get_group_request::GetGroupRequest;
use crate::http::aws::iam::types::list_group_policies_request::ListGroupPoliciesRequest;
use crate::http::aws::iam::types::list_groups_for_user_request::ListGroupsForUserRequest;
use crate::http::aws::iam::types::list_groups_request::ListGroupsRequest;
use crate::http::aws::iam::types::put_group_policy_request::PutGroupPolicyRequest;
use crate::http::aws::iam::{constants, db};

pub(crate) async fn create_group(
    ctx: &OperationCtx, input: &CreateGroupRequest, db: &LocalDb,
) -> Result<CreateGroupOutput, OperationError> {
    input.validate("$")?;

    let current_time = Utc::now().timestamp();

    let mut tx = db.new_tx().await?;
    let group_id = create_resource_id(&mut tx, constants::group::PREFIX, ResourceType::Group).await?;

    let mut insert_group = prepare_group_for_insert(ctx, input, &group_id, current_time)
        .map_err(|err| OperationError::new(ApiErrorKind::ServiceFailure, err.to_string().as_str()))?;

    db::group::create(&mut tx, &mut insert_group).await?;

    let group = Group::builder()
        .group_id(group_id)
        .group_name(&insert_group.group_name)
        .path(&insert_group.path)
        .arn(&insert_group.arn)
        .create_date(DateTime::from_secs(insert_group.create_date))
        .build()
        .unwrap();
    let output = CreateGroupOutput::builder().group(group).build();

    tx.commit().await?;
    Ok(output)
}

fn prepare_group_for_insert(
    ctx: &OperationCtx, input: &CreateGroupRequest, group_id: &str, current_time: i64,
) -> Result<InsertGroup, InsertGroupBuilderError> {
    let path = input.path().unwrap_or("/");
    let group_name = input.group_name().unwrap().trim();
    let arn = format!("arn:aws:iam::{:0>12}:group{}{}", ctx.account_id, path, group_name);
    InsertGroupBuilder::default()
        .id(None) // The property will be initialized during insert
        .account_id(ctx.account_id)
        .path(path.to_owned())
        .arn(arn)
        .group_name(group_name.to_owned())
        .group_id(group_id.to_owned())
        .create_date(current_time)
        .build()
}

pub(crate) async fn list_groups(
    ctx: &OperationCtx, input: &ListGroupsRequest, db: &LocalDb,
) -> Result<ListGroupsOutput, OperationError> {
    input.validate("$")?;

    let query: ListGroupsQuery = input.into();

    // obtain connection
    let mut connection = db.new_connection().await?;

    let found_groups: Vec<SelectGroup> = db::group::list(connection.as_mut(), ctx.account_id, &query).await?;

    let marker = super::common::create_encoded_marker(&query, found_groups.len())?;
    let groups = super::common::convert_and_limit(&found_groups, query.limit).unwrap_or_default();

    let output = ListGroupsOutput::builder()
        .set_groups(Some(groups))
        .set_is_truncated(marker.as_ref().map(|_v| true))
        .set_marker(marker)
        .build()
        .unwrap();
    Ok(output)
}

pub(crate) async fn find_by_name<'a, E>(
    ctx: &OperationCtx, executor: E, group_name: &str,
) -> Result<SelectGroup, OperationError>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    match db::group::find_by_name(executor, ctx.account_id, group_name).await? {
        Some(group) => Ok(group),
        None => {
            return Err(OperationError::new(
                ApiErrorKind::NoSuchEntity,
                format!("IAM group with name '{}' doesn't exist.", group_name).as_str(),
            ));
        }
    }
}

pub(crate) async fn find_id_by_name<'a, E>(
    executor: E, account_id: i64, group_name: &str,
) -> Result<i64, OperationError>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    match db::group::find_id_by_name(executor, account_id, group_name).await? {
        Some(id) => Ok(id),
        None => {
            return Err(OperationError::new(
                ApiErrorKind::NoSuchEntity,
                format!("IAM group with name '{}' doesn't exist.", group_name).as_str(),
            ));
        }
    }
}

pub(crate) async fn add_user_to_group(
    ctx: &OperationCtx, input: &AddUserToGroupRequest, db: &LocalDb,
) -> Result<AddUserToGroupOutput, OperationError> {
    input.validate("$")?;
    let mut tx = db.new_tx().await?;

    let found_group = find_by_name(ctx, tx.as_mut(), input.group_name().unwrap().trim()).await?;
    let found_user = super::user::find_by_name(ctx, tx.as_mut(), input.user_name().unwrap().trim()).await?;
    db::group::assign_user_to_group(&mut tx, found_group.id, found_user.id).await?;
    let output = AddUserToGroupOutput::builder().build();

    tx.commit().await?;
    Ok(output)
}

pub(crate) async fn list_groups_for_user(
    ctx: &OperationCtx, input: &ListGroupsForUserRequest, db: &LocalDb,
) -> Result<ListGroupsForUserOutput, OperationError> {
    input.validate("$")?;

    let mut connection = db.new_tx().await?;

    let user_id = super::user::find_id_by_name(connection.as_mut(), ctx.account_id, input.user_name().unwrap()).await?;

    let query = ListGroupsByUserQuery {
        user_id,
        limit: match input.max_items() {
            None => 10,
            Some(v) => *v,
        },
        skip: match input.marker_type() {
            None => 0,
            // unwrap is safe since marker must be validated before DB query preparation
            Some(marker_type) => marker_type.marker().unwrap().truncate_amount,
        },
    };

    let found_groups = db::group::find_by_user_id(connection.as_mut(), &query).await?;

    let groups = super::common::convert_and_limit(&found_groups, query.limit);
    let marker = super::common::create_encoded_marker(&query, found_groups.len())?;

    let output = ListGroupsForUserOutput::builder()
        .set_groups(groups)
        .set_is_truncated(marker.as_ref().map(|_v| true))
        .set_marker(marker)
        .build()
        .unwrap();
    Ok(output)
}

pub(crate) async fn attach_group_policy(
    ctx: &OperationCtx, input: &AttachGroupPolicyRequest, db: &LocalDb,
) -> Result<AttachGroupPolicyOutput, OperationError> {
    input.validate("$")?;

    let mut tx = db.new_tx().await?;

    let found_group = find_by_name(ctx, tx.as_mut(), input.group_name().unwrap().trim()).await?;
    let found_policy_id =
        super::policy::find_id_by_arn(tx.as_mut(), ctx.account_id, input.policy_arn().unwrap().trim()).await?;

    db::group::assign_policy_to_group(&mut tx, found_group.id, found_policy_id).await?;

    let output = AttachGroupPolicyOutput::builder().build();

    tx.commit().await?;
    Ok(output)
}

pub(crate) async fn get_group(
    ctx: &OperationCtx, input: &GetGroupRequest, db: &LocalDb,
) -> Result<GetGroupOutput, OperationError> {
    input.validate("$")?;

    let group_name = input.group_name().unwrap().trim();
    // obtain connection
    let mut connection = db.new_connection().await?;

    match db::group::find_by_name(connection.as_mut(), ctx.account_id, group_name).await? {
        None => {
            return Err(OperationError::new(
                ApiErrorKind::NoSuchEntity,
                format!("IAM group with name '{}' doesn't exist.", group_name).as_str(),
            ));
        }
        Some(group) => {
            let limit = match input.max_items() {
                None => 10,
                Some(v) => *v,
            };

            let skip = match input.marker_type() {
                None => 0,
                Some(marker_type) => marker_type.marker().unwrap().truncate_amount,
            };

            let query = ListUsersByGroupQuery {
                group_id: group.id,
                limit,
                skip,
            };

            let found_users = db::user::find_by_group_id(connection.as_mut(), &query).await?;

            let marker = super::common::create_encoded_marker(&query, found_users.len())?;

            let mut users = vec![];
            for found_user in found_users {
                let user = (&found_user).into();
                users.push(user);
            }

            let output = GetGroupOutput::builder()
                .group((&group).into())
                .set_users(Some(users))
                .set_is_truncated(marker.as_ref().map(|_v| true))
                .set_marker(marker)
                .build()
                .unwrap();
            Ok(output)
        }
    }
}

pub(crate) async fn get_group_policy(
    ctx: &OperationCtx, input: &GetGroupPolicyRequest, db: &LocalDb,
) -> Result<GetGroupPolicyOutput, OperationError> {
    input.validate("$")?;

    let mut connection = db.new_connection().await?;

    let group_name = input.group_name().unwrap().trim();
    let group_id = find_id_by_name(connection.as_mut(), ctx.account_id, group_name).await?;

    let policy_name = input.policy_name().unwrap().trim();
    let inline_policy =
        db::group_inline_policy::find_by_group_id_and_name(connection.as_mut(), group_id, policy_name).await?;

    match inline_policy {
        None => Err(OperationError::new(
            ApiErrorKind::NoSuchEntity,
            format!("IAM inline policy with name '{policy_name}' not found for group with name '{group_name}'.")
                .as_str(),
        )),
        Some(policy) => {
            let output = GetGroupPolicyOutput::builder()
                .group_name(group_name)
                .policy_name(&policy.policy_name)
                .policy_document(&policy.policy_document)
                .build()
                .unwrap();
            Ok(output)
        }
    }
}

pub(crate) async fn put_group_policy(
    ctx: &OperationCtx, input: &PutGroupPolicyRequest, db: &LocalDb,
) -> Result<PutGroupPolicyOutput, OperationError> {
    input.validate("$")?;

    let mut tx = db.new_tx().await?;

    let group_id = find_id_by_name(tx.as_mut(), ctx.account_id, input.group_name().unwrap().trim()).await?;

    let mut inline_policy =
        DbInlinePolicy::new(group_id, input.policy_name().unwrap(), input.policy_document().unwrap());

    db::group_inline_policy::save(&mut tx, &mut inline_policy).await?;

    let output = PutGroupPolicyOutput::builder().build();

    tx.commit().await?;
    Ok(output)
}

pub(crate) async fn list_group_policies(
    ctx: &OperationCtx, input: &ListGroupPoliciesRequest, db: &LocalDb,
) -> Result<ListGroupPoliciesOutput, OperationError> {
    input.validate("$")?;

    let mut connection = db.new_connection().await?;

    let group_name = input.group_name().unwrap().trim();
    let group_id = find_id_by_name(connection.as_mut(), ctx.account_id, group_name).await?;

    let query = ListInlinePoliciesQuery::new(group_id, input.max_items(), input.marker_type());
    let found_policies = db::group_inline_policy::find_by_group_id(connection.as_mut(), &query).await?;

    let policy_names = super::common::convert_and_limit(&found_policies, query.limit);
    let marker = super::common::create_encoded_marker(&query, found_policies.len())?;

    let output = ListGroupPoliciesOutput::builder()
        .set_policy_names(policy_names)
        .set_is_truncated(marker.as_ref().map(|_v| true))
        .set_marker(marker)
        .build()
        .unwrap();
    Ok(output)
}
