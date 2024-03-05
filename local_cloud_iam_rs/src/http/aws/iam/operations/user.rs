use aws_sdk_iam::operation::attach_user_policy::AttachUserPolicyOutput;
use aws_sdk_iam::operation::create_user::CreateUserOutput;
use aws_sdk_iam::operation::delete_user::DeleteUserOutput;
use aws_sdk_iam::operation::delete_user_permissions_boundary::DeleteUserPermissionsBoundaryOutput;
use aws_sdk_iam::operation::delete_user_policy::DeleteUserPolicyOutput;
use aws_sdk_iam::operation::detach_user_policy::DetachUserPolicyOutput;
use aws_sdk_iam::operation::get_user::GetUserOutput;
use aws_sdk_iam::operation::get_user_policy::GetUserPolicyOutput;
use aws_sdk_iam::operation::list_attached_user_policies::ListAttachedUserPoliciesOutput;
use aws_sdk_iam::operation::list_user_policies::ListUserPoliciesOutput;
use aws_sdk_iam::operation::list_user_tags::ListUserTagsOutput;
use aws_sdk_iam::operation::list_users::ListUsersOutput;
use aws_sdk_iam::operation::put_user_permissions_boundary::PutUserPermissionsBoundaryOutput;
use aws_sdk_iam::operation::put_user_policy::PutUserPolicyOutput;
use aws_sdk_iam::operation::tag_user::TagUserOutput;
use aws_sdk_iam::operation::untag_user::UntagUserOutput;
use aws_sdk_iam::operation::update_user::UpdateUserOutput;
use aws_sdk_iam::types::{AttachedPermissionsBoundary, PermissionsBoundaryAttachmentType, User};
use aws_smithy_types::DateTime;
use chrono::Utc;
use sqlx::{Executor, Sqlite, Transaction};

use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::actions::error::ApiErrorKind;
use crate::http::aws::iam::db::types::common::ListByIdQuery;
use crate::http::aws::iam::db::types::inline_policy::DbInlinePolicy;
use crate::http::aws::iam::db::types::resource_identifier::ResourceType;
use crate::http::aws::iam::db::types::tags::ListTagsQuery;
use crate::http::aws::iam::db::types::user::{
    InsertUser, InsertUserBuilder, InsertUserBuilderError, SelectUser, UpdateUserQuery,
};
use crate::http::aws::iam::operations::common::create_resource_id;
use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::ActionError;
use crate::http::aws::iam::types::attach_user_policy::AttachUserPolicyRequest;
use crate::http::aws::iam::types::create_user::CreateUserRequest;
use crate::http::aws::iam::types::delete_user::DeleteUserRequest;
use crate::http::aws::iam::types::delete_user_permissions_boundary::DeleteUserPermissionsBoundaryRequest;
use crate::http::aws::iam::types::delete_user_policy::DeleteUserPolicyRequest;
use crate::http::aws::iam::types::detach_user_policy::DetachUserPolicyRequest;
use crate::http::aws::iam::types::get_user::GetUserRequest;
use crate::http::aws::iam::types::get_user_policy::GetUserPolicyRequest;
use crate::http::aws::iam::types::list_attached_user_policies::ListAttachedUserPoliciesRequest;
use crate::http::aws::iam::types::list_user_policies::ListUserPoliciesRequest;
use crate::http::aws::iam::types::list_user_tags::ListUserTagsRequest;
use crate::http::aws::iam::types::list_users::ListUsersRequest;
use crate::http::aws::iam::types::put_user_permissions_boundary::PutUserPermissionsBoundaryRequest;
use crate::http::aws::iam::types::put_user_policy::PutUserPolicyRequest;
use crate::http::aws::iam::types::tag_user::TagUserRequest;
use crate::http::aws::iam::types::untag_user::UntagUserRequest;
use crate::http::aws::iam::types::update_user::UpdateUserRequest;
use crate::http::aws::iam::{constants, db};

pub async fn create_user<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &CreateUserRequest,
) -> Result<CreateUserOutput, ActionError> {
    input.validate("$")?;
    let current_time = Utc::now().timestamp();

    let user_id = create_resource_id(tx, constants::user::PREFIX, ResourceType::User).await?;

    let policy_id = match input.permissions_boundary() {
        None => None,
        Some(permissions_boundary) => {
            Some(super::policy::find_id_by_arn(tx.as_mut(), ctx.account_id, permissions_boundary).await?)
        }
    };

    let mut insert_user = prepare_user_for_insert(ctx, input, &user_id, policy_id, current_time)
        .map_err(|err| ActionError::new(ApiErrorKind::ServiceFailure, err.to_string().as_str()))?;

    db::user::create(tx, &mut insert_user).await?;

    let mut user_tags = super::tag::prepare_for_db(input.tags(), insert_user.id.unwrap());
    db::Tags::User.save_all(tx, &mut user_tags).await?;

    let permissions_boundary = match policy_id {
        None => None,
        Some(_) => Some(
            AttachedPermissionsBoundary::builder()
                .permissions_boundary_type(PermissionsBoundaryAttachmentType::Policy)
                .permissions_boundary_arn(input.permissions_boundary().unwrap())
                .build(),
        ),
    };

    let user = User::builder()
        .path(&insert_user.path)
        .user_name(&insert_user.username)
        .user_id(&insert_user.user_id)
        .arn(&insert_user.arn)
        .create_date(DateTime::from_secs(insert_user.create_date))
        .set_permissions_boundary(permissions_boundary)
        .set_tags(super::tag::prepare_for_output(&user_tags))
        .build()
        .unwrap();
    let output = CreateUserOutput::builder().user(user).build();
    Ok(output)
}

fn prepare_user_for_insert(
    ctx: &OperationCtx, input: &CreateUserRequest, user_id: &str, policy_id: Option<i64>, current_time: i64,
) -> Result<InsertUser, InsertUserBuilderError> {
    let path = input.path().unwrap_or("/");
    let username = input.user_name().unwrap().trim();
    let arn = format!("arn:aws:iam::{:0>12}:user{}{}", ctx.account_id, path, username);
    InsertUserBuilder::default()
        .id(None)
        .account_id(ctx.account_id)
        .username(input.user_name().unwrap().to_owned())
        .arn(arn)
        .path(path.to_owned())
        .user_id(user_id.to_owned())
        .policy_id(policy_id)
        .create_date(current_time)
        .build()
}

pub(crate) async fn find_by_name<'a, E>(
    ctx: &OperationCtx, executor: E, user_name: &str,
) -> Result<SelectUser, ActionError>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    match db::user::find_by_name(executor, ctx.account_id, user_name).await? {
        Some(user) => Ok(user),
        None => {
            return Err(ActionError::new(
                ApiErrorKind::NoSuchEntity,
                format!("IAM user with name '{}' doesn't exist.", user_name).as_str(),
            ));
        }
    }
}

pub(crate) async fn find_id_by_name<'a, E>(executor: E, account_id: i64, user_name: &str) -> Result<i64, ActionError>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    match db::user::find_id_by_name(executor, account_id, user_name).await? {
        Some(id) => Ok(id),
        None => {
            return Err(ActionError::new(
                ApiErrorKind::NoSuchEntity,
                format!("IAM user with name '{}' doesn't exist.", user_name).as_str(),
            ));
        }
    }
}

pub(crate) async fn attach_user_policy<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &AttachUserPolicyRequest,
) -> Result<AttachUserPolicyOutput, ActionError> {
    input.validate("$")?;

    let user_name = input.user_name().unwrap();
    let found_user_id = find_id_by_name(tx.as_mut(), ctx.account_id, user_name).await?;

    let policy_arn = input.policy_arn().unwrap();
    let found_policy_id = super::policy::find_id_by_arn(tx.as_mut(), ctx.account_id, policy_arn).await?;

    db::user::assign_policy_to_user(tx.as_mut(), found_user_id, found_policy_id).await?;

    let output = AttachUserPolicyOutput::builder().build();
    Ok(output)
}

pub(crate) async fn list_users<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &ListUsersRequest,
) -> Result<ListUsersOutput, ActionError> {
    input.validate("$")?;

    let query = input.into();

    let found_users: Vec<SelectUser> = db::user::list(tx.as_mut(), ctx.account_id, &query).await?;

    let users = super::common::convert_and_limit(&found_users, query.limit).unwrap_or_default();
    let marker = super::common::create_encoded_marker(&query, found_users.len())?;

    let output = ListUsersOutput::builder()
        .set_users(Some(users))
        .set_is_truncated(marker.as_ref().map(|_v| true))
        .set_marker(marker)
        .build()
        .unwrap();
    Ok(output)
}

pub(crate) async fn list_user_tags<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &ListUserTagsRequest,
) -> Result<ListUserTagsOutput, ActionError> {
    input.validate("$")?;

    let found_user_id = find_id_by_name(tx.as_mut(), ctx.account_id, input.user_name().unwrap().trim()).await?;

    let query = ListTagsQuery::new(input.max_items(), input.marker_type());
    let found_tags = db::Tags::User.list(tx.as_mut(), found_user_id, &query).await?;

    let tags = super::common::convert_and_limit(&found_tags, query.limit);
    let marker = super::common::create_encoded_marker(&query, found_tags.len())?;

    let output = ListUserTagsOutput::builder()
        .set_tags(tags)
        .set_is_truncated(marker.as_ref().map(|_v| true))
        .set_marker(marker)
        .build()
        .unwrap();
    Ok(output)
}

pub(crate) async fn tag_user<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &TagUserRequest,
) -> Result<TagUserOutput, ActionError> {
    input.validate("$")?;

    let user_id = find_id_by_name(tx.as_mut(), ctx.account_id, input.user_name().unwrap().trim()).await?;
    let mut user_tags = super::tag::prepare_for_db(input.tags(), user_id);

    db::Tags::User.save_all(tx, &mut user_tags).await?;
    let count = db::Tags::User.count(tx.as_mut(), user_id).await?;
    if count > constants::tag::MAX_COUNT {
        return Err(ActionError::new(
            ApiErrorKind::LimitExceeded,
            format!("Cannot assign more than {} tags to IAM user.", constants::tag::MAX_COUNT).as_str(),
        ));
    }

    let output = TagUserOutput::builder().build();
    Ok(output)
}

pub(crate) async fn get_user_policy<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &GetUserPolicyRequest,
) -> Result<GetUserPolicyOutput, ActionError> {
    input.validate("$")?;

    let user_name = input.user_name().unwrap().trim();
    let user_id = find_id_by_name(tx.as_mut(), ctx.account_id, user_name).await?;

    let policy_name = input.policy_name().unwrap().trim();
    let inline_policy = db::user_inline_policy::find_by_user_id_and_name(tx.as_mut(), user_id, policy_name).await?;

    match inline_policy {
        None => Err(ActionError::new(
            ApiErrorKind::NoSuchEntity,
            format!("IAM inline policy with name '{policy_name}' not found for user with name '{user_name}'.").as_str(),
        )),
        Some(policy) => {
            let output = GetUserPolicyOutput::builder()
                .user_name(user_name)
                .policy_name(&policy.policy_name)
                .policy_document(&policy.policy_document)
                .build()
                .unwrap();
            Ok(output)
        }
    }
}

pub(crate) async fn put_user_policy<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &PutUserPolicyRequest,
) -> Result<PutUserPolicyOutput, ActionError> {
    input.validate("$")?;

    let user_id = find_id_by_name(tx.as_mut(), ctx.account_id, input.user_name().unwrap().trim()).await?;

    let mut inline_policy =
        DbInlinePolicy::new(user_id, input.policy_name().unwrap(), input.policy_document().unwrap());

    db::user_inline_policy::save(tx, &mut inline_policy).await?;

    let output = PutUserPolicyOutput::builder().build();
    Ok(output)
}

pub(crate) async fn list_user_policies<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &ListUserPoliciesRequest,
) -> Result<ListUserPoliciesOutput, ActionError> {
    input.validate("$")?;

    let user_name = input.user_name().unwrap().trim();
    let user_id = find_id_by_name(tx.as_mut(), ctx.account_id, user_name).await?;

    let query = ListByIdQuery::new(user_id, input.max_items(), input.marker_type());
    let found_policies = db::user_inline_policy::find_by_user_id(tx.as_mut(), &query).await?;

    let policy_names = super::common::convert_and_limit(&found_policies, query.limit);
    let marker = super::common::create_encoded_marker(&query, found_policies.len())?;

    let output = ListUserPoliciesOutput::builder()
        .set_policy_names(policy_names)
        .set_is_truncated(marker.as_ref().map(|_v| true))
        .set_marker(marker)
        .build()
        .unwrap();
    Ok(output)
}

pub(crate) async fn untag_user<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &UntagUserRequest,
) -> Result<UntagUserOutput, ActionError> {
    input.validate("$")?;

    let user_id = find_id_by_name(tx.as_mut(), ctx.account_id, input.user_name().unwrap().trim()).await?;

    db::Tags::User.delete_all(tx, user_id, &input.tag_keys()).await?;

    let output = UntagUserOutput::builder().build();
    Ok(output)
}

pub(crate) async fn update_user<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &UpdateUserRequest,
) -> Result<UpdateUserOutput, ActionError> {
    input.validate("$")?;

    let query = UpdateUserQuery {
        user_name: input.user_name().unwrap().to_owned(),
        new_path: input.new_path().map(|s| s.to_owned()),
        new_user_name: input.new_user_name().map(|s| s.to_owned()),
    };
    let result = db::user::update(tx.as_mut(), ctx.account_id, &query).await?;
    if !result {
        return Err(ActionError::new(ApiErrorKind::NoSuchEntity, "Entity does not exist."));
    }

    let output = UpdateUserOutput::builder().build();
    Ok(output)
}

pub(crate) async fn get_user<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &GetUserRequest,
) -> Result<GetUserOutput, ActionError> {
    input.validate("$")?;

    let output = GetUserOutput::builder().build();
    Ok(output)
}

pub(crate) async fn put_user_permissions_boundary<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &PutUserPermissionsBoundaryRequest,
) -> Result<PutUserPermissionsBoundaryOutput, ActionError> {
    input.validate("$")?;

    let policy_arn = input.permissions_boundary().unwrap();
    let policy_id = super::policy::find_id_by_arn(tx.as_mut(), ctx.account_id, policy_arn).await?;

    let user_name = input.user_name().unwrap();
    let is_updated = db::user::update_permissions_boundary(tx.as_mut(), ctx.account_id, user_name, policy_id).await?;
    if !is_updated {
        // There is only one reason why `is_updated == false` - user doesn't exist.
        return Err(ActionError::new(
            ApiErrorKind::NoSuchEntity,
            format!("IAM user with name '{}' doesn't exist.", user_name).as_str(),
        ));
    }
    let output = PutUserPermissionsBoundaryOutput::builder().build();
    Ok(output)
}

pub(crate) async fn delete_user_permissions_boundary<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &DeleteUserPermissionsBoundaryRequest,
) -> Result<DeleteUserPermissionsBoundaryOutput, ActionError> {
    input.validate("$")?;

    let user_name = input.user_name().unwrap();
    let is_updated = db::user::delete_permissions_boundary(tx.as_mut(), ctx.account_id, user_name).await?;
    if !is_updated {
        // There is only one reason why `is_updated == false` - user doesn't exist.
        return Err(ActionError::new(
            ApiErrorKind::NoSuchEntity,
            format!("IAM user with name '{}' doesn't exist.", user_name).as_str(),
        ));
    }

    let output = DeleteUserPermissionsBoundaryOutput::builder().build();
    Ok(output)
}

pub(crate) async fn list_attached_user_policies<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &ListAttachedUserPoliciesRequest,
) -> Result<ListAttachedUserPoliciesOutput, ActionError> {
    input.validate("$")?;

    let output = ListAttachedUserPoliciesOutput::builder().build();
    Ok(output)
}

pub(crate) async fn delete_user<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &DeleteUserRequest,
) -> Result<DeleteUserOutput, ActionError> {
    input.validate("$")?;

    let output = DeleteUserOutput::builder().build();
    Ok(output)
}

pub(crate) async fn delete_user_policy<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &DeleteUserPolicyRequest,
) -> Result<DeleteUserPolicyOutput, ActionError> {
    input.validate("$")?;

    let output = DeleteUserPolicyOutput::builder().build();
    Ok(output)
}

pub(crate) async fn detach_user_policy<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &DetachUserPolicyRequest,
) -> Result<DetachUserPolicyOutput, ActionError> {
    input.validate("$")?;
    let user_name = input.user_name().unwrap();
    let user_id = find_id_by_name(tx.as_mut(), ctx.account_id, user_name).await?;

    let policy_arn = input.policy_arn().unwrap();
    let policy_id = super::policy::find_id_by_arn(tx.as_mut(), ctx.account_id, policy_arn).await?;

    let is_updated = db::user::detach_policy(tx, user_id, policy_id).await?;
    if !is_updated {
        return Err(ActionError::new(ApiErrorKind::InvalidInput, "Policy is not attached to the user."));
    }
    let output = DetachUserPolicyOutput::builder().build();
    Ok(output)
}
