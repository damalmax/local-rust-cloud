use aws_sdk_iam::operation::delete_signing_certificate::DeleteSigningCertificateOutput;
use aws_sdk_iam::operation::list_signing_certificates::ListSigningCertificatesOutput;
use aws_sdk_iam::operation::update_signing_certificate::UpdateSigningCertificateOutput;
use aws_sdk_iam::operation::upload_signing_certificate::UploadSigningCertificateOutput;
use aws_sdk_iam::types::{SigningCertificate, StatusType};
use aws_smithy_types::DateTime;
use chrono::Utc;
use uuid::Uuid;

use local_cloud_db::LocalDb;
use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::db;
use crate::http::aws::iam::db::types::signing_certificate::InsertSigningCertificate;
use crate::http::aws::iam::db::types::signing_certificate_status_type::SigningCertificateStatusType;
use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::types::delete_signing_certificate::DeleteSigningCertificateRequest;
use crate::http::aws::iam::types::list_signing_certificates::ListSigningCertificatesRequest;
use crate::http::aws::iam::types::update_signing_certificate::UpdateSigningCertificateRequest;
use crate::http::aws::iam::types::upload_signing_certificate::UploadSigningCertificateRequest;

pub(crate) async fn upload_signing_certificate(
    ctx: &OperationCtx, input: &UploadSigningCertificateRequest, db: &LocalDb,
) -> Result<UploadSigningCertificateOutput, OperationError> {
    input.validate("$")?;

    let mut tx = db.new_tx().await?;
    let current_time = Utc::now().timestamp();

    let user = super::user::find_by_name(ctx, tx.as_mut(), input.user_name().unwrap().trim()).await?;
    let cert_content = input.certificate_body().unwrap();
    let x509 = input.certificate_body_type().unwrap().metadata().unwrap();

    let certificate_id = Uuid::new_v4().to_string().replace("-", "").to_uppercase();

    let status = if current_time <= x509.validity.not_after.timestamp()
        && current_time >= x509.validity.not_before.timestamp()
    {
        SigningCertificateStatusType::Active
    } else {
        SigningCertificateStatusType::Inactive // consider to use 'Expired' status
    };

    let mut insert_signing_certificate = InsertSigningCertificate {
        id: None,
        account_id: ctx.account_id,
        certificate_id,
        certificate_body: cert_content.to_owned(),
        status,
        upload_date: current_time,
        user_id: user.id,
    };

    db::sighing_certificate::create(&mut tx, &mut insert_signing_certificate).await?;

    let certificate = SigningCertificate::builder()
        .certificate_id(&insert_signing_certificate.certificate_id)
        .certificate_body(cert_content)
        .user_name(&user.username)
        .upload_date(DateTime::from_secs(current_time))
        .status(StatusType::Active)
        .build()
        .unwrap();

    let output = UploadSigningCertificateOutput::builder()
        .certificate(certificate)
        .build();

    tx.commit().await?;

    Ok(output)
}

pub(crate) async fn update_signing_certificate(
    ctx: &OperationCtx, input: &UpdateSigningCertificateRequest, db: &LocalDb,
) -> Result<UpdateSigningCertificateOutput, OperationError> {
    input.validate("$")?;

    let output = UpdateSigningCertificateOutput::builder().build();
    Ok(output)
}

pub(crate) async fn list_signing_certificates(
    ctx: &OperationCtx, input: &ListSigningCertificatesRequest, db: &LocalDb,
) -> Result<ListSigningCertificatesOutput, OperationError> {
    input.validate("$")?;

    let output = ListSigningCertificatesOutput::builder().build().unwrap();
    Ok(output)
}

pub(crate) async fn delete_signing_certificate(
    ctx: &OperationCtx, input: &DeleteSigningCertificateRequest, db: &LocalDb,
) -> Result<DeleteSigningCertificateOutput, OperationError> {
    input.validate("$")?;

    let output = DeleteSigningCertificateOutput::builder().build();
    Ok(output)
}
