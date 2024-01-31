use aws_sdk_iam::operation::attach_user_policy::AttachUserPolicyOutput;
use aws_sdk_iam::operation::create_user::CreateUserOutput;
use aws_sdk_iam::types::{AttachedPermissionsBoundary, PermissionsBoundaryAttachmentType, User};
use aws_smithy_types::DateTime;
use chrono::Utc;
use sqlx::{Executor, Sqlite};

use local_cloud_db::LocalDb;
use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::actions::error::ApiErrorKind;
use crate::http::aws::iam::db::types::resource_identifier::ResourceType;
use crate::http::aws::iam::db::types::user::{InsertUser, InsertUserBuilder, InsertUserBuilderError, SelectUser};
use crate::http::aws::iam::operations::common::create_resource_id;
use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::types::attach_user_policy_request::AttachUserPolicyRequest;
use crate::http::aws::iam::types::create_user_request::CreateUserRequest;
use crate::http::aws::iam::{constants, db};

pub async fn create_user(
    ctx: &OperationCtx, input: &CreateUserRequest, db: &LocalDb,
) -> Result<CreateUserOutput, OperationError> {
    input.validate("$")?;
    let current_time = Utc::now().timestamp();

    let mut tx = db.new_tx().await?;
    let user_id = create_resource_id(&mut tx, constants::user::PREFIX, ResourceType::User).await?;

    let policy_id = super::policy::find_policy_id_by_arn(tx.as_mut(), input.permissions_boundary()).await?;

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
            ))
        }
    }
}

pub(crate) async fn attach_user_policy(
    ctx: &OperationCtx, input: &AttachUserPolicyRequest, db: &LocalDb,
) -> Result<AttachUserPolicyOutput, OperationError> {
    input.validate("$")?;

    let mut tx = db.new_tx().await?;

    let user_name = input.user_name().unwrap();
    let found_user_id = match db::user::find_id_by_name(tx.as_mut(), ctx.account_id, user_name).await? {
        Some(user_id) => user_id,
        None => {
            return Err(OperationError::new(
                ApiErrorKind::NoSuchEntity,
                format!("IAM user with name '{}' doesn't exist.", user_name).as_str(),
            ))
        }
    };

    let policy_arn = input.policy_arn().unwrap();
    let found_policy_id = match db::policy::find_id_by_arn(tx.as_mut(), policy_arn).await? {
        Some(policy_id) => policy_id,
        None => {
            return Err(OperationError::new(
                ApiErrorKind::NoSuchEntity,
                format!("Unable to find policy with ARN '{}'.", policy_arn).as_str(),
            ))
        }
    };

    db::user::assign_policy_to_user(&mut tx, found_user_id, found_policy_id).await?;

    let output = AttachUserPolicyOutput::builder().build();

    tx.commit().await?;
    Ok(output)
}
