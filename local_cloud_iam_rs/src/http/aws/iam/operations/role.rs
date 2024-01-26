use aws_sdk_iam::operation::create_role::CreateRoleOutput;
use aws_sdk_iam::types::{Role, Tag};
use aws_smithy_types::DateTime;
use chrono::Utc;

use local_cloud_db::LocalDb;
use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::actions::error::ApiErrorKind;
use crate::http::aws::iam::db::types::resource_identifier::ResourceType;
use crate::http::aws::iam::db::types::role::{InsertRole, InsertRoleBuilder, InsertRoleBuilderError};
use crate::http::aws::iam::db::types::role_tag::DbRoleTag;
use crate::http::aws::iam::operations::common::create_resource_id;
use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::types::create_role_request::CreateRoleRequest;
use crate::http::aws::iam::{constants, db, types};

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

    let mut role_tags = prepare_tags_for_insert(input.tags(), insert_role.id.unwrap());
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
        .set_tags(prepare_tags_for_output(role_tags))
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

fn prepare_tags_for_insert(tags: Option<&[types::tag::Tag]>, user_id: i64) -> Vec<DbRoleTag> {
    match tags {
        None => vec![],
        Some(tags) => {
            let mut role_tags = vec![];
            for tag in tags {
                let role_tag = DbRoleTag::new(user_id, tag.key().unwrap(), tag.value().unwrap_or(""));
                role_tags.push(role_tag);
            }
            role_tags
        }
    }
}

fn prepare_tags_for_output(tags: Vec<DbRoleTag>) -> Option<Vec<Tag>> {
    if tags.len() == 0 {
        None
    } else {
        Some(tags.iter().map(|tag| tag.into()).collect())
    }
}
