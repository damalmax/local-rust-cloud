use aws_sdk_iam::operation::upload_server_certificate::UploadServerCertificateOutput;
use aws_sdk_iam::types::ServerCertificateMetadata;
use aws_smithy_types::DateTime;
use chrono::Utc;
use x509_parser::pem::parse_x509_pem;

use local_cloud_db::LocalDb;
use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::db::types::resource_identifier::ResourceType;
use crate::http::aws::iam::db::types::server_certificate::InsertServerCertificate;
use crate::http::aws::iam::operations::common::create_resource_id;
use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::types::upload_server_certificate_request::UploadServerCertificateRequest;
use crate::http::aws::iam::{constants, db};

pub(crate) async fn upload_server_certificate(
    ctx: &OperationCtx, input: &UploadServerCertificateRequest, db: &LocalDb,
) -> Result<UploadServerCertificateOutput, OperationError> {
    input.validate("$")?;

    let mut tx = db.new_tx().await?;

    let current_time = Utc::now().timestamp();
    let path = input.path().unwrap_or("/").trim();
    let server_certificate_name = input.server_certificate_name().unwrap().trim();
    let arn = format!("arn:aws:iam::{:0>12}:server-certificate{}{}", ctx.account_id, path, server_certificate_name);
    let certificate_body = input.certificate_body().unwrap();
    let pem = parse_x509_pem(certificate_body.as_bytes()).unwrap().1;
    let cert = pem.parse_x509().unwrap();

    let server_certificate_id =
        create_resource_id(&mut tx, constants::server_certificate::PREFIX, ResourceType::ServerCertificate).await?;

    let mut insert_server_certificate = InsertServerCertificate {
        id: None,
        account_id: ctx.account_id,
        arn,
        path: path.to_owned(),
        certificate_body: certificate_body.to_owned(),
        certificate_chain: input.certificate_chain().map(|s| s.to_owned()),
        server_certificate_name: server_certificate_name.to_owned(),
        server_certificate_id,
        upload_date: current_time,
        expiration: cert.validity.not_after.timestamp(),
    };

    db::server_certificate::create(&mut tx, &mut insert_server_certificate).await?;

    let mut server_certificate_tags =
        super::tag::prepare_for_insert(input.tags(), insert_server_certificate.id.unwrap());

    db::Tags::ServerCertificate
        .save_all(&mut tx, &mut server_certificate_tags)
        .await?;

    let server_certificate_metadata = ServerCertificateMetadata::builder()
        .arn(&insert_server_certificate.arn)
        .path(path)
        .server_certificate_id(&insert_server_certificate.server_certificate_id)
        .server_certificate_name(server_certificate_name)
        .expiration(DateTime::from_secs(insert_server_certificate.expiration))
        .upload_date(DateTime::from_secs(current_time))
        .build()
        .unwrap();

    let output = UploadServerCertificateOutput::builder()
        .server_certificate_metadata(server_certificate_metadata)
        .set_tags(super::tag::prepare_for_output(&server_certificate_tags))
        .build();

    tx.commit().await?;

    Ok(output)
}
