use aws_sdk_iam::operation::attach_group_policy::AttachGroupPolicyOutput;
use aws_sdk_iam::operation::attach_role_policy::AttachRolePolicyOutput;
use aws_sdk_iam::operation::create_role::CreateRoleOutput;
use aws_sdk_iam::types::Role;
use aws_smithy_types::DateTime;
use chrono::Utc;

use local_cloud_db::LocalDb;
use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::actions::error::ApiErrorKind;
use crate::http::aws::iam::db::types::resource_identifier::ResourceType;
use crate::http::aws::iam::db::types::role::{InsertRole, InsertRoleBuilder, InsertRoleBuilderError};
use crate::http::aws::iam::operations::common::create_resource_id;
use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::types::attach_group_policy_request::AttachGroupPolicyRequest;
use crate::http::aws::iam::types::attach_role_policy_request::AttachRolePolicyRequest;
use crate::http::aws::iam::types::create_role_request::CreateRoleRequest;
use crate::http::aws::iam::{constants, db};

pub async fn create_role(
    ctx: &OperationCtx, input: &CreateRoleRequest, db: &LocalDb,
) -> Result<CreateRoleOutput, OperationError> {
    input.validate("$")?;

    let current_time = Utc::now().timestamp();

    let mut tx = db.new_tx().await?;
    let role_id = create_resource_id(&mut tx, constants::role::PREFIX, ResourceType::Role).await?;

    let policy_id = super::policy::find_policy_id_by_arn((&mut tx).as_mut(), input.permissions_boundary()).await?;
    let mut insert_role = prepare_role_for_insert(ctx, input, &role_id, policy_id, current_time)
        .map_err(|err| OperationError::new(ApiErrorKind::ServiceFailure, err.to_string().as_str()))?;

    db::role::create(&mut tx, &mut insert_role).await?;

    let mut role_tags = super::common::prepare_tags_for_insert(input.tags(), insert_role.id.unwrap());
    db::role_tag::save_all(&mut tx, &mut role_tags).await?;

    let role = Role::builder()
        .role_id(role_id)
        .set_assume_role_policy_document(
            input
                .assume_role_policy_document()
                .map(|s| urlencoding::encode(s).to_string()),
        )
        .role_name(&insert_role.role_name)
        .path(&insert_role.path)
        .arn(&insert_role.arn)
        .set_description(insert_role.description.as_ref().map(|s| s.to_owned()))
        .create_date(DateTime::from_secs(insert_role.create_date))
        .set_tags(super::common::prepare_tags_for_output(&role_tags))
        .build()
        .unwrap();
    let output = CreateRoleOutput::builder().role(role).build();

    tx.commit().await?;
    Ok(output)
}

fn prepare_role_for_insert(
    ctx: &OperationCtx, input: &CreateRoleRequest, role_id: &str, policy_id: Option<i64>, current_time: i64,
) -> Result<InsertRole, InsertRoleBuilderError> {
    let role_name = input.role_name().unwrap().trim();
    let arn = format!("arn:aws:iam::{:0>12}:role/{}", ctx.account_id, role_name);
    let max_session_duration = input
        .max_session_duration()
        .map(|v| v.clone())
        .unwrap_or(constants::role::DEFAULT_MAX_SESSION_DURATION) as i64;
    InsertRoleBuilder::default()
        .id(None)
        .account_id(ctx.account_id)
        .role_name(role_name.to_owned())
        .description(input.description().map(|s| s.to_owned()))
        .max_session_duration(max_session_duration)
        .arn(arn)
        .path(input.path().unwrap_or("/").to_owned())
        .role_id(role_id.to_owned())
        .policy_id(policy_id)
        .create_date(current_time)
        .build()
}

pub(crate) async fn attach_role_policy(
    ctx: &OperationCtx, input: &AttachRolePolicyRequest, db: &LocalDb,
) -> Result<AttachRolePolicyOutput, OperationError> {
    input.validate("$")?;

    let mut tx = db.new_tx().await?;

    let found_role = db::role::find_id_by_name((&mut tx).as_mut(), ctx.account_id, input.role_name().unwrap()).await?;
    if found_role.is_none() {
        return Err(OperationError::new(
            ApiErrorKind::NoSuchEntity,
            format!("IAM role with name '{}' doesn't exist.", input.role_name().unwrap().trim()).as_str(),
        ));
    }

    let found_policy = db::policy::find_id_by_arn((&mut tx).as_mut(), input.policy_arn().unwrap()).await?;
    if found_policy.is_none() {
        return Err(OperationError::new(
            ApiErrorKind::NoSuchEntity,
            format!("Unable to find policy with ARN '{}'.", input.policy_arn().unwrap()).as_str(),
        ));
    }

    db::role::assign_policy_to_role(&mut tx, found_role.unwrap(), found_policy.unwrap()).await?;

    let output = AttachRolePolicyOutput::builder().build();

    tx.commit().await?;
    Ok(output)
}
