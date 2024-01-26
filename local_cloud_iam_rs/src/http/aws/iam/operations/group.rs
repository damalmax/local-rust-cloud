use aws_sdk_iam::operation::create_group::CreateGroupOutput;
use aws_sdk_iam::types::Group;
use aws_smithy_types::DateTime;
use chrono::Utc;

use local_cloud_db::LocalDb;
use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::actions::error::ApiErrorKind;
use crate::http::aws::iam::db::types::group::{InsertGroup, InsertGroupBuilder, InsertGroupBuilderError};
use crate::http::aws::iam::db::types::resource_identifier::ResourceType;
use crate::http::aws::iam::operations::common::create_resource_id;
use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::types::create_group_request::CreateGroupRequest;
use crate::http::aws::iam::{constants, db};

pub async fn create_group(
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
        // The property will be initialized during insert
        .id(None)
        .account_id(ctx.account_id)
        .path(input.path().unwrap_or("/").to_owned())
        .arn(arn)
        .group_name(group_name.to_owned())
        .group_id(group_id.to_owned())
        .create_date(current_time)
        .build()
}
