use aws_sdk_iam::operation::add_user_to_group::AddUserToGroupOutput;
use aws_sdk_iam::operation::attach_group_policy::AttachGroupPolicyOutput;
use aws_sdk_iam::operation::create_group::CreateGroupOutput;
use aws_sdk_iam::operation::get_group::GetGroupOutput;
use aws_sdk_iam::operation::list_groups::ListGroupsOutput;
use aws_sdk_iam::types::Group;
use aws_smithy_types::DateTime;
use chrono::Utc;
use sqlx::{Executor, Sqlite};

use local_cloud_db::LocalDb;
use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::actions::error::ApiErrorKind;
use crate::http::aws::iam::db::types::group::{InsertGroup, InsertGroupBuilder, InsertGroupBuilderError, SelectGroup};
use crate::http::aws::iam::db::types::resource_identifier::ResourceType;
use crate::http::aws::iam::db::types::user::ListUsersByGroupQuery;
use crate::http::aws::iam::operations::common::create_resource_id;
use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::types::add_user_to_group_request::AddUserToGroupRequest;
use crate::http::aws::iam::types::attach_group_policy_request::AttachGroupPolicyRequest;
use crate::http::aws::iam::types::create_group_request::CreateGroupRequest;
use crate::http::aws::iam::types::get_group_request::GetGroupRequest;
use crate::http::aws::iam::types::list_groups_request::ListGroupsRequest;
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
    let group_name = input.group_name().unwrap().trim();
    let arn = format!("arn:aws:iam::{:0>12}:group/{}", ctx.account_id, group_name);
    InsertGroupBuilder::default()
        .id(None) // The property will be initialized during insert
        .account_id(ctx.account_id)
        .path(input.path().unwrap_or("/").to_owned())
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

    let query = input.into();

    // obtain connection
    let mut connection = db.new_connection().await?;

    let found_groups: Vec<SelectGroup> = db::group::list_groups(connection.as_mut(), &query).await?;
    let marker = super::common::create_encoded_marker(&query, found_groups.len())?;

    let mut groups: Vec<Group> = vec![];
    for i in 0..(query.limit) {
        let group = found_groups.get(i as usize);
        match group {
            None => break,
            Some(select_group) => {
                groups.push(select_group.into());
            }
        }
    }

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
            ))
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

pub(crate) async fn attach_group_policy(
    ctx: &OperationCtx, input: &AttachGroupPolicyRequest, db: &LocalDb,
) -> Result<AttachGroupPolicyOutput, OperationError> {
    input.validate("$")?;

    let mut tx = db.new_tx().await?;

    let found_group = find_by_name(ctx, tx.as_mut(), input.group_name().unwrap().trim()).await?;
    let found_policy_id = super::policy::find_id_by_arn(tx.as_mut(), input.policy_arn().unwrap().trim()).await?;

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
            ))
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
                let user = found_user.into();
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
