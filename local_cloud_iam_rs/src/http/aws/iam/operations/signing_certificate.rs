use aws_sdk_iam::operation::delete_signing_certificate::DeleteSigningCertificateOutput;
use aws_sdk_iam::operation::list_signing_certificates::ListSigningCertificatesOutput;
use aws_sdk_iam::operation::update_signing_certificate::UpdateSigningCertificateOutput;
use aws_sdk_iam::operation::upload_signing_certificate::UploadSigningCertificateOutput;
use aws_sdk_iam::types::{SigningCertificate, StatusType};
use aws_smithy_types::DateTime;
use chrono::Utc;
use sqlx::{Sqlite, Transaction};
use uuid::Uuid;

use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::actions::error::ApiErrorKind;
use crate::http::aws::iam::db;
use crate::http::aws::iam::db::types::common::ListByIdQuery;
use crate::http::aws::iam::db::types::signing_certificate::{InsertSigningCertificate, UpdateSigningCertificateQuery};
use crate::http::aws::iam::db::types::signing_certificate_status_type::SigningCertificateStatusType;
use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::ActionError;
use crate::http::aws::iam::types::delete_signing_certificate::DeleteSigningCertificateRequest;
use crate::http::aws::iam::types::list_signing_certificates::ListSigningCertificatesRequest;
use crate::http::aws::iam::types::update_signing_certificate::UpdateSigningCertificateRequest;
use crate::http::aws::iam::types::upload_signing_certificate::UploadSigningCertificateRequest;

pub(crate) async fn upload_signing_certificate<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &UploadSigningCertificateRequest,
) -> Result<UploadSigningCertificateOutput, ActionError> {
    input.validate("$")?;

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

    db::sighing_certificate::create(tx, &mut insert_signing_certificate).await?;

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
    Ok(output)
}

pub(crate) async fn update_signing_certificate<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &UpdateSigningCertificateRequest,
) -> Result<UpdateSigningCertificateOutput, ActionError> {
    input.validate("$")?;

    let user_name = input.user_name().unwrap();
    let user_id = super::user::find_id_by_name(tx.as_mut(), ctx.account_id, user_name).await?;

    let query = UpdateSigningCertificateQuery {
        user_id,
        status: input.status().unwrap().into(),
        certificate_id: input.certificate_id().unwrap().to_owned(),
    };
    let result = db::sighing_certificate::update(tx.as_mut(), &query).await?;
    if !result {
        return Err(ActionError::new(ApiErrorKind::NoSuchEntity, "Entity does not exist."));
    }

    let output = UpdateSigningCertificateOutput::builder().build();
    Ok(output)
}

pub(crate) async fn list_signing_certificates<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &ListSigningCertificatesRequest,
) -> Result<ListSigningCertificatesOutput, ActionError> {
    input.validate("$")?;

    let user_name = input.user_name().unwrap();
    let user_id = super::user::find_id_by_name(tx.as_mut(), ctx.account_id, user_name).await?;

    let query = ListByIdQuery::new(user_id, input.max_items(), input.marker_type());

    let found_certificates = db::sighing_certificate::find_by_user_id(tx.as_mut(), &query).await?;
    let certificates = super::common::convert_and_limit(&found_certificates, query.limit);
    let marker = super::common::create_encoded_marker(&query, found_certificates.len())?;

    let output = ListSigningCertificatesOutput::builder()
        .set_certificates(certificates)
        .set_is_truncated(marker.as_ref().map(|_v| true))
        .set_marker(marker)
        .build()
        .unwrap();
    Ok(output)
}

pub(crate) async fn delete_signing_certificate<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &DeleteSigningCertificateRequest,
) -> Result<DeleteSigningCertificateOutput, ActionError> {
    input.validate("$")?;

    let output = DeleteSigningCertificateOutput::builder().build();
    Ok(output)
}
