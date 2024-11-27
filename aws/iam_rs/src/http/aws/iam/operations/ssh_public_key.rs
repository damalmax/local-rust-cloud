use aws_sdk_iam::operation::delete_ssh_public_key::DeleteSshPublicKeyOutput;
use aws_sdk_iam::operation::get_ssh_public_key::GetSshPublicKeyOutput;
use aws_sdk_iam::operation::list_ssh_public_keys::ListSshPublicKeysOutput;
use aws_sdk_iam::operation::update_ssh_public_key::UpdateSshPublicKeyOutput;
use aws_sdk_iam::operation::upload_ssh_public_key::UploadSshPublicKeyOutput;
use aws_sdk_iam::types::{SshPublicKey, StatusType};
use aws_smithy_types::DateTime;
use chrono::Utc;
use sqlx::{Sqlite, Transaction};

use validators::NamedValidator;

use crate::http::aws::iam::actions::error::ApiErrorKind;
use crate::http::aws::iam::db::types::common::ListByIdQuery;
use crate::http::aws::iam::db::types::resource_identifier::ResourceType;
use crate::http::aws::iam::db::types::ssh_public_key::{InsertSshPublicKey, UpdateSshPublicKeyQuery};
use crate::http::aws::iam::db::types::ssh_public_key_type::SshPublicKeyStatusType;
use crate::http::aws::iam::operations::common::create_resource_id;
use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::ActionError;
use crate::http::aws::iam::types::delete_ssh_public_key::DeleteSshPublicKeyRequest;
use crate::http::aws::iam::types::get_ssh_public_key::GetSshPublicKeyRequest;
use crate::http::aws::iam::types::list_ssh_public_keys::ListSshPublicKeysRequest;
use crate::http::aws::iam::types::update_ssh_public_key::UpdateSshPublicKeyRequest;
use crate::http::aws::iam::types::upload_ssh_public_key::UploadSshPublicKeyRequest;
use crate::http::aws::iam::{constants, db};

pub(crate) async fn upload_ssh_public_key<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &UploadSshPublicKeyRequest,
) -> Result<UploadSshPublicKeyOutput, ActionError> {
    input.validate("$")?;

    let current_time = Utc::now().timestamp();
    let user = super::user::find_by_name(ctx, tx.as_mut(), input.user_name().unwrap().trim()).await?;

    let ssh_public_key_body = input.ssh_public_key_body().unwrap().trim();
    let ssh_public_key_id =
        create_resource_id(tx, constants::ssh_public_key::PREFIX, ResourceType::SshPublicKey).await?;

    let mut insert_ssh_public_key = InsertSshPublicKey {
        id: None,
        user_id: user.id,
        key_id: ssh_public_key_id,
        body: ssh_public_key_body.to_owned(),
        status: SshPublicKeyStatusType::Active,
        upload_date: current_time,
    };
    db::ssh_public_key::upload(tx, &mut insert_ssh_public_key)
        .await
        .map_err(|_err| ActionError::new(ApiErrorKind::DuplicateSshPublicKey,
                                         "The request was rejected because the SSH public key is already associated with the specified IAM user."))?;

    let parsed_public_key = openssh_keys::PublicKey::parse(ssh_public_key_body)
        .map_err(|err| ActionError::new(ApiErrorKind::InvalidInput, "Invalid SSH public key supplied."))?;

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
    Ok(output)
}

pub(crate) async fn update_ssh_public_key<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &UpdateSshPublicKeyRequest,
) -> Result<UpdateSshPublicKeyOutput, ActionError> {
    input.validate("$")?;

    let user_id = super::user::find_id_by_name(tx.as_mut(), ctx.account_id, input.user_name().unwrap()).await?;

    let query = UpdateSshPublicKeyQuery {
        key_id: input.ssh_public_key_id().unwrap().to_string(),
        status: input.status().unwrap().into(),
        user_id,
    };
    let result = db::ssh_public_key::update(tx.as_mut(), &query).await?;
    if !result {
        return Err(ActionError::new(ApiErrorKind::NoSuchEntity, "Entity does not exist."));
    }

    let output = UpdateSshPublicKeyOutput::builder().build();
    Ok(output)
}

pub(crate) async fn get_ssh_public_key<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &GetSshPublicKeyRequest,
) -> Result<GetSshPublicKeyOutput, ActionError> {
    input.validate("$")?;

    let output = GetSshPublicKeyOutput::builder().build();
    Ok(output)
}

pub(crate) async fn list_ssh_public_keys<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &ListSshPublicKeysRequest,
) -> Result<ListSshPublicKeysOutput, ActionError> {
    input.validate("$")?;

    let user_name = input.user_name().unwrap(); // TODO: if user_name is not provided, obtain the info from request
    let user_id = super::user::find_id_by_name(tx.as_mut(), ctx.account_id, user_name).await?;
    let query = ListByIdQuery::new(user_id, input.max_items(), input.marker_type());

    let found_keys = db::ssh_public_key::find_by_user_id(tx.as_mut(), &query).await?;

    let keys = super::common::convert_and_limit(&found_keys, query.limit);
    let marker = super::common::create_encoded_marker(&query, found_keys.len())?;

    let output = ListSshPublicKeysOutput::builder()
        .set_ssh_public_keys(keys)
        .set_is_truncated(marker.as_ref().map(|_v| true))
        .set_marker(marker)
        .build();
    Ok(output)
}

pub(crate) async fn delete_ssh_public_key<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &DeleteSshPublicKeyRequest,
) -> Result<DeleteSshPublicKeyOutput, ActionError> {
    input.validate("$")?;

    let output = DeleteSshPublicKeyOutput::builder().build();
    Ok(output)
}
