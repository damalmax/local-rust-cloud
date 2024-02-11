use aws_sdk_iam::operation::attach_user_policy::AttachUserPolicyOutput;
use aws_sdk_iam::operation::create_user::CreateUserOutput;
use aws_sdk_iam::operation::get_user_policy::GetUserPolicyOutput;
use aws_sdk_iam::operation::list_user_tags::ListUserTagsOutput;
use aws_sdk_iam::operation::list_users::ListUsersOutput;
use aws_sdk_iam::operation::put_user_policy::PutUserPolicyOutput;
use aws_sdk_iam::operation::tag_user::TagUserOutput;
use aws_sdk_iam::types::{AttachedPermissionsBoundary, PermissionsBoundaryAttachmentType, Tag, User};
use aws_smithy_types::DateTime;
use chrono::Utc;
use sqlx::{Executor, Sqlite};

use local_cloud_db::LocalDb;
use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::actions::error::ApiErrorKind;
use crate::http::aws::iam::db::types::inline_policy::DbInlinePolicy;
use crate::http::aws::iam::db::types::resource_identifier::ResourceType;
use crate::http::aws::iam::db::types::tags::ListTagsQuery;
use crate::http::aws::iam::db::types::user::{InsertUser, InsertUserBuilder, InsertUserBuilderError, SelectUser};
use crate::http::aws::iam::operations::common::create_resource_id;
use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::types::attach_user_policy_request::AttachUserPolicyRequest;
use crate::http::aws::iam::types::create_user_request::CreateUserRequest;
use crate::http::aws::iam::types::get_user_policy_request::GetUserPolicyRequest;
use crate::http::aws::iam::types::list_user_tags_request::ListUserTagsRequest;
use crate::http::aws::iam::types::list_users_request::ListUsersRequest;
use crate::http::aws::iam::types::put_user_policy_request::PutUserPolicyRequest;
use crate::http::aws::iam::types::tag_user_request::TagUserRequest;
use crate::http::aws::iam::{constants, db};

pub async fn create_user(
    ctx: &OperationCtx, input: &CreateUserRequest, db: &LocalDb,
) -> Result<CreateUserOutput, OperationError> {
    input.validate("$")?;
    let current_time = Utc::now().timestamp();

    let mut tx = db.new_tx().await?;
    let user_id = create_resource_id(&mut tx, constants::user::PREFIX, ResourceType::User).await?;

    let policy_id =
        super::policy::find_policy_id_by_arn(tx.as_mut(), ctx.account_id, input.permissions_boundary()).await?;

    let mut insert_user = prepare_user_for_insert(ctx, input, &user_id, policy_id, current_time)
        .map_err(|err| OperationError::new(ApiErrorKind::ServiceFailure, err.to_string().as_str()))?;

    db::user::create(&mut tx, &mut insert_user).await?;

    let mut user_tags = super::common::prepare_tags_for_insert(input.tags(), insert_user.id.unwrap());
    db::user_tag::save_all(&mut tx, &mut user_tags).await?;

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
        .set_tags(super::common::prepare_tags_for_output(&user_tags))
        .build()
        .unwrap();
    let output = CreateUserOutput::builder().user(user).build();
    tx.commit().await?;

    Ok(output)
}

fn prepare_user_for_insert(
    ctx: &OperationCtx, input: &CreateUserRequest, user_id: &str, policy_id: Option<i64>, current_time: i64,
) -> Result<InsertUser, InsertUserBuilderError> {
    let username = input.user_name().unwrap().trim();
    let arn = format!("arn:aws:iam::{:0>12}:user/{}", ctx.account_id, username);
    InsertUserBuilder::default()
        .id(None)
        .account_id(ctx.account_id)
        .username(input.user_name().unwrap().to_owned())
        .arn(arn)
        .path(input.path().unwrap_or("/").to_owned())
        .user_id(user_id.to_owned())
        .policy_id(policy_id)
        .create_date(current_time)
        .build()
}

pub(crate) async fn find_by_name<'a, E>(
    ctx: &OperationCtx, executor: E, user_name: &str,
) -> Result<SelectUser, OperationError>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    match db::user::find_by_name(executor, ctx.account_id, user_name).await? {
        Some(user) => Ok(user),
        None => {
            return Err(OperationError::new(
                ApiErrorKind::NoSuchEntity,
                format!("IAM user with name '{}' doesn't exist.", user_name).as_str(),
            ));
        }
    }
}

pub(crate) async fn find_id_by_name<'a, E>(executor: E, account_id: i64, user_name: &str) -> Result<i64, OperationError>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    match db::user::find_id_by_name(executor, account_id, user_name).await? {
        Some(id) => Ok(id),
        None => {
            return Err(OperationError::new(
                ApiErrorKind::NoSuchEntity,
                format!("IAM user with name '{}' doesn't exist.", user_name).as_str(),
            ));
        }
    }
}

pub(crate) async fn attach_user_policy(
    ctx: &OperationCtx, input: &AttachUserPolicyRequest, db: &LocalDb,
) -> Result<AttachUserPolicyOutput, OperationError> {
    input.validate("$")?;

    let mut tx = db.new_tx().await?;

    let user_name = input.user_name().unwrap();
    let found_user_id = find_id_by_name(tx.as_mut(), ctx.account_id, user_name).await?;

    let policy_arn = input.policy_arn().unwrap();
    let found_policy_id = super::policy::find_id_by_arn(tx.as_mut(), ctx.account_id, policy_arn).await?;

    db::user::assign_policy_to_user(&mut tx, found_user_id, found_policy_id).await?;

    let output = AttachUserPolicyOutput::builder().build();

    tx.commit().await?;
    Ok(output)
}

pub(crate) async fn list_users(
    ctx: &OperationCtx, input: &ListUsersRequest, db: &LocalDb,
) -> Result<ListUsersOutput, OperationError> {
    input.validate("$")?;

    let mut connection = db.new_connection().await?;

    let query = input.into();

    let found_users: Vec<SelectUser> = db::user::list(connection.as_mut(), ctx.account_id, &query).await?;
    let marker = super::common::create_encoded_marker(&query, found_users.len())?;

    let mut users: Vec<User> = vec![];
    for i in 0..(query.limit) {
        let user = found_users.get(i as usize);
        match user {
            None => break,
            Some(select_user) => {
                users.push(select_user.into());
            }
        }
    }

    let output = ListUsersOutput::builder()
        .set_users(Some(users))
        .set_is_truncated(marker.as_ref().map(|_v| true))
        .set_marker(marker)
        .build()
        .unwrap();

    Ok(output)
}

pub(crate) async fn list_user_tags(
    ctx: &OperationCtx, input: &ListUserTagsRequest, db: &LocalDb,
) -> Result<ListUserTagsOutput, OperationError> {
    input.validate("$")?;

    // obtain connection
    let mut connection = db.new_connection().await?;

    let found_user_id = find_id_by_name(connection.as_mut(), ctx.account_id, input.user_name().unwrap().trim()).await?;

    let query = ListTagsQuery::new(input.max_items(), input.marker_type());
    let found_tags = db::user_tag::list_tags(connection.as_mut(), found_user_id, &query).await?;
    let marker = super::common::create_encoded_marker(&query, found_tags.len())?;

    let mut tags: Vec<Tag> = vec![];
    for i in 0..(query.limit) {
        let found_tag = found_tags.get(i as usize);
        match found_tag {
            None => break,
            Some(tag) => {
                tags.push(tag.into());
            }
        }
    }

    let output = ListUserTagsOutput::builder()
        .set_tags(Some(tags))
        .set_is_truncated(marker.as_ref().map(|_v| true))
        .set_marker(marker)
        .build()
        .unwrap();
    Ok(output)
}

pub(crate) async fn tag_user(
    ctx: &OperationCtx, input: &TagUserRequest, db: &LocalDb,
) -> Result<TagUserOutput, OperationError> {
    input.validate("$")?;

    let mut tx = db.new_tx().await?;

    let user_id = find_id_by_name(tx.as_mut(), ctx.account_id, input.user_name().unwrap().trim()).await?;
    let mut user_tags = super::common::prepare_tags_for_insert(input.tags(), user_id);

    db::user_tag::save_all(&mut tx, &mut user_tags).await?;
    let count = db::user_tag::count(tx.as_mut(), user_id).await?;
    if count > constants::tag::MAX_COUNT {
        return Err(OperationError::new(
            ApiErrorKind::LimitExceeded,
            format!("Cannot assign more than {} tags to IAM user.", constants::tag::MAX_COUNT).as_str(),
        ));
    }

    let output = TagUserOutput::builder().build();

    tx.commit().await?;

    Ok(output)
}

pub(crate) async fn get_user_policy(
    ctx: &OperationCtx, input: &GetUserPolicyRequest, db: &LocalDb,
) -> Result<GetUserPolicyOutput, OperationError> {
    input.validate("$")?;

    let mut connection = db.new_connection().await?;

    let user_name = input.user_name().unwrap().trim();
    let user_id = find_id_by_name(connection.as_mut(), ctx.account_id, user_name).await?;

    let policy_name = input.policy_name().unwrap().trim();
    let inline_policy =
        db::user_inline_policy::find_by_user_id_and_name(connection.as_mut(), user_id, policy_name).await?;

    match inline_policy {
        None => Err(OperationError::new(
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

pub(crate) async fn put_user_policy(
    ctx: &OperationCtx, input: &PutUserPolicyRequest, db: &LocalDb,
) -> Result<PutUserPolicyOutput, OperationError> {
    input.validate("$")?;

    let mut tx = db.new_tx().await?;

    let user_id = find_id_by_name(tx.as_mut(), ctx.account_id, input.user_name().unwrap().trim()).await?;

    let mut inline_policy =
        DbInlinePolicy::new(user_id, input.policy_name().unwrap(), input.policy_document().unwrap());

    db::user_inline_policy::save(&mut tx, &mut inline_policy).await?;

    let output = PutUserPolicyOutput::builder().build();

    tx.commit().await?;
    Ok(output)
}
