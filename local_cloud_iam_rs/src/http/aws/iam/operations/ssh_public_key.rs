use aws_sdk_iam::operation::upload_ssh_public_key::UploadSshPublicKeyOutput;
use aws_sdk_iam::types::{SshPublicKey, StatusType};
use aws_smithy_types::DateTime;
use chrono::Utc;

use local_cloud_db::LocalDb;
use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::actions::error::ApiErrorKind;
use crate::http::aws::iam::db::types::resource_identifier::ResourceType;
use crate::http::aws::iam::db::types::ssh_public_key::InsertSshPublicKey;
use crate::http::aws::iam::db::types::ssh_public_key_type::SshPublicKeyStatusType;
use crate::http::aws::iam::operations::common::create_resource_id;
use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
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
