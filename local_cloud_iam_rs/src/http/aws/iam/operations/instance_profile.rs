use aws_sdk_iam::operation::add_role_to_instance_profile::AddRoleToInstanceProfileOutput;
use aws_sdk_iam::operation::create_instance_profile::CreateInstanceProfileOutput;
use aws_sdk_iam::types::InstanceProfile;
use aws_smithy_types::DateTime;
use chrono::Utc;
use sqlx::{Executor, Sqlite};

use local_cloud_db::LocalDb;
use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::actions::error::ApiErrorKind;
use crate::http::aws::iam::db::types::instance_profile::InsertInstanceProfile;
use crate::http::aws::iam::db::types::resource_identifier::ResourceType;
use crate::http::aws::iam::operations::common::create_resource_id;
use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::types::add_role_to_instance_profile_request::AddRoleToInstanceProfileRequest;
use crate::http::aws::iam::types::create_instance_profile_request::CreateInstanceProfileRequest;
use crate::http::aws::iam::{constants, db};

pub(crate) async fn create_instance_profile(
    ctx: &OperationCtx, input: &CreateInstanceProfileRequest, db: &LocalDb,
) -> Result<CreateInstanceProfileOutput, OperationError> {
    input.validate("$")?;

    let current_time = Utc::now().timestamp();
    let mut tx = db.new_tx().await?;

    let instance_profile_name = input.instance_profile_name().unwrap();
    let arn = format!("arn:aws:iam::{:0>12}:instance-profile/{}", ctx.account_id, instance_profile_name);
    let instance_profile_id =
        create_resource_id(&mut tx, constants::instance_profile::PREFIX, ResourceType::InstanceProfile).await?;

    let mut insert_instance_profile = InsertInstanceProfile {
        id: None,
        account_id: ctx.account_id,
        instance_profile_name: instance_profile_name.to_owned(),
        instance_profile_id,
        arn,
        path: input.path().unwrap_or("/").to_owned(),
        create_date: current_time,
    };

    db::instance_profile::create(&mut tx, &mut insert_instance_profile).await?;

    let mut tags = super::common::prepare_tags_for_insert(input.tags(), insert_instance_profile.id.unwrap());
    db::instance_profile_tag::save_all(&mut tx, &mut tags).await?;

    let instance_profile = InstanceProfile::builder()
        .instance_profile_name(&insert_instance_profile.instance_profile_name)
        .instance_profile_id(&insert_instance_profile.instance_profile_id)
        .path(&insert_instance_profile.path)
        .arn(&insert_instance_profile.arn)
        .create_date(DateTime::from_secs(current_time))
        .set_roles(Some(Vec::with_capacity(0))) // no roles when just create an instance profile.
        .set_tags(super::common::prepare_tags_for_output(&tags))
        .build()
        .unwrap();

    let output = CreateInstanceProfileOutput::builder()
        .instance_profile(instance_profile)
        .build();

    tx.commit().await?;
    Ok(output)
}

pub(crate) async fn find_id_by_name<'a, E>(
    ctx: &OperationCtx, executor: E, instance_profile_name: &str,
) -> Result<i64, OperationError>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    match db::instance_profile::find_id_by_name(executor, ctx.account_id, instance_profile_name).await? {
        Some(id) => Ok(id),
        None => {
            return Err(OperationError::new(
                ApiErrorKind::NoSuchEntity,
                format!("IAM instance profile with name '{}' doesn't exist.", instance_profile_name).as_str(),
            ))
        }
    }
}

pub(crate) async fn add_role_to_instance_profile(
    ctx: &OperationCtx, input: &AddRoleToInstanceProfileRequest, db: &LocalDb,
) -> Result<AddRoleToInstanceProfileOutput, OperationError> {
    input.validate("$")?;

    let mut tx = db.new_tx().await?;
    let instance_profile_id = find_id_by_name(ctx, tx.as_mut(), input.instance_profile_name().unwrap().trim()).await?;
    let role_id = super::role::find_id_by_name(ctx, tx.as_mut(), input.role_name().unwrap().trim()).await?;

    db::instance_profile::assign_role_to_instance_profile(&mut tx, instance_profile_id, role_id).await?;

    let output = AddRoleToInstanceProfileOutput::builder().build();

    tx.commit().await?;
    Ok(output)
}
