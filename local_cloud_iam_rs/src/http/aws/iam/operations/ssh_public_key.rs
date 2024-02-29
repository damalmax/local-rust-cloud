use aws_sdk_iam::operation::delete_ssh_public_key::DeleteSshPublicKeyOutput;
use aws_sdk_iam::operation::get_ssh_public_key::GetSshPublicKeyOutput;
use aws_sdk_iam::operation::list_ssh_public_keys::ListSshPublicKeysOutput;
use aws_sdk_iam::operation::update_ssh_public_key::UpdateSshPublicKeyOutput;
use aws_sdk_iam::operation::upload_ssh_public_key::UploadSshPublicKeyOutput;
use aws_sdk_iam::types::{SshPublicKey, StatusType};
use aws_smithy_types::DateTime;
use chrono::Utc;

use local_cloud_db::LocalDb;
use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::actions::error::ApiErrorKind;
use crate::http::aws::iam::db::types::resource_identifier::ResourceType;
use crate::http::aws::iam::db::types::ssh_public_key::{InsertSshPublicKey, UpdateSshPublicKeyQuery};
use crate::http::aws::iam::db::types::ssh_public_key_type::SshPublicKeyStatusType;
use crate::http::aws::iam::operations::common::create_resource_id;
use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::types::delete_ssh_public_key::DeleteSshPublicKeyRequest;
use crate::http::aws::iam::types::get_ssh_public_key::GetSshPublicKeyRequest;
use crate::http::aws::iam::types::list_ssh_public_keys::ListSshPublicKeysRequest;
use crate::http::aws::iam::types::update_ssh_public_key::UpdateSshPublicKeyRequest;
use crate::http::aws::iam::types::upload_ssh_public_key::UploadSshPublicKeyRequest;
use crate::http::aws::iam::{constants, db};

pub(crate) async fn upload_ssh_public_key(
    ctx: &OperationCtx, input: &UploadSshPublicKeyRequest, db: &LocalDb,
) -> Result<UploadSshPublicKeyOutput, OperationError> {
    input.validate("$")?;

    let mut tx = db.new_tx().await?;

    let current_time = Utc::now().timestamp();
    let user = super::user::find_by_name(ctx, tx.as_mut(), input.user_name().unwrap().trim()).await?;

    let ssh_public_key_body = input.ssh_public_key_body().unwrap().trim();
    let ssh_public_key_id =
        create_resource_id(&mut tx, constants::ssh_public_key::PREFIX, ResourceType::SshPublicKey).await?;

    let mut insert_ssh_public_key = InsertSshPublicKey {
        id: None,
        user_id: user.id,
        key_id: ssh_public_key_id,
        body: ssh_public_key_body.to_owned(),
        status: SshPublicKeyStatusType::Active,
        upload_date: current_time,
    };
    db::ssh_public_key::upload(&mut tx, &mut insert_ssh_public_key)
        .await
        .map_err(|_err| OperationError::new(ApiErrorKind::DuplicateSshPublicKey,
                                            "The request was rejected because the SSH public key is already associated with the specified IAM user."))?;

    let parsed_public_key = openssh_keys::PublicKey::parse(ssh_public_key_body)
        .map_err(|err| OperationError::new(ApiErrorKind::InvalidInput, "Invalid SSH public key supplied."))?;

    let ssh_public_key = SshPublicKey::builder()
        .user_name(&user.username)
        .status(StatusType::Active)
        .ssh_public_key_body(&insert_ssh_public_key.body)
        .ssh_public_key_id(&insert_ssh_public_key.key_id)
        .fingerprint(parsed_public_key.fingerprint())
        .upload_date(DateTime::from_secs(current_time))
        .build()
        .unwrap();

    let output = UploadSshPublicKeyOutput::builder()
        .ssh_public_key(ssh_public_key)
        .build();

    tx.commit().await?;

    Ok(output)
}

pub(crate) async fn update_ssh_public_key(
    ctx: &OperationCtx, input: &UpdateSshPublicKeyRequest, db: &LocalDb,
) -> Result<UpdateSshPublicKeyOutput, OperationError> {
    input.validate("$")?;

    let mut tx = db.new_tx().await?;

    let user_id = super::user::find_id_by_name(tx.as_mut(), ctx.account_id, input.user_name().unwrap()).await?;

    let query = UpdateSshPublicKeyQuery {
        key_id: input.ssh_public_key_id().unwrap().to_string(),
        status: input.status().unwrap().into(),
        user_id,
    };
    let result = db::ssh_public_key::update(tx.as_mut(), &query).await?;
    if !result {
        return Err(OperationError::new(ApiErrorKind::NoSuchEntity, "Entity does not exist."));
    }

    let output = UpdateSshPublicKeyOutput::builder().build();
    tx.commit().await?;
    Ok(output)
}

pub(crate) async fn get_ssh_public_key(
    ctx: &OperationCtx, input: &GetSshPublicKeyRequest, db: &LocalDb,
) -> Result<GetSshPublicKeyOutput, OperationError> {
    input.validate("$")?;

    let output = GetSshPublicKeyOutput::builder().build();

    Ok(output)
}

pub(crate) async fn list_ssh_public_keys(
    ctx: &OperationCtx, input: &ListSshPublicKeysRequest, db: &LocalDb,
) -> Result<ListSshPublicKeysOutput, OperationError> {
    input.validate("$")?;

    let output = ListSshPublicKeysOutput::builder().build();

    Ok(output)
}

pub(crate) async fn delete_ssh_public_key(
    ctx: &OperationCtx, input: &DeleteSshPublicKeyRequest, db: &LocalDb,
) -> Result<DeleteSshPublicKeyOutput, OperationError> {
    input.validate("$")?;

    let mut tx = db.new_tx().await?;

    let output = DeleteSshPublicKeyOutput::builder().build();

    tx.commit().await?;
    Ok(output)
}
