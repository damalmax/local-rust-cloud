use aws_sdk_iam::operation::delete_server_certificate::DeleteServerCertificateOutput;
use aws_sdk_iam::operation::get_server_certificate::GetServerCertificateOutput;
use aws_sdk_iam::operation::list_server_certificate_tags::ListServerCertificateTagsOutput;
use aws_sdk_iam::operation::list_server_certificates::ListServerCertificatesOutput;
use aws_sdk_iam::operation::tag_server_certificate::TagServerCertificateOutput;
use aws_sdk_iam::operation::untag_server_certificate::UntagServerCertificateOutput;
use aws_sdk_iam::operation::update_server_certificate::UpdateServerCertificateOutput;
use aws_sdk_iam::operation::upload_server_certificate::UploadServerCertificateOutput;
use aws_sdk_iam::types::ServerCertificateMetadata;
use aws_smithy_types::DateTime;
use chrono::Utc;
use sqlx::{Executor, Sqlite};
use x509_parser::pem::parse_x509_pem;

use local_cloud_db::LocalDb;
use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::actions::error::ApiErrorKind;
use crate::http::aws::iam::db::types::resource_identifier::ResourceType;
use crate::http::aws::iam::db::types::server_certificate::InsertServerCertificate;
use crate::http::aws::iam::db::types::tags::ListTagsQuery;
use crate::http::aws::iam::operations::common::create_resource_id;
use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::types::delete_server_certificate::DeleteServerCertificateRequest;
use crate::http::aws::iam::types::get_server_certificate::GetServerCertificateRequest;
use crate::http::aws::iam::types::list_server_certificate_tags::ListServerCertificateTagsRequest;
use crate::http::aws::iam::types::list_server_certificates::ListServerCertificatesRequest;
use crate::http::aws::iam::types::tag_server_certificate::TagServerCertificateRequest;
use crate::http::aws::iam::types::untag_server_certificate::UntagServerCertificateRequest;
use crate::http::aws::iam::types::update_server_certificate::UpdateServerCertificateRequest;
use crate::http::aws::iam::types::upload_server_certificate::UploadServerCertificateRequest;
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

    let mut server_certificate_tags = super::tag::prepare_for_db(input.tags(), insert_server_certificate.id.unwrap());

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

pub(crate) async fn find_id_by_name<'a, E>(
    executor: E, account_id: i64, server_certificate_name: &str,
) -> Result<i64, OperationError>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    match db::server_certificate::find_id_by_name(executor, account_id, server_certificate_name).await? {
        Some(role_id) => Ok(role_id),
        None => {
            return Err(OperationError::new(
                ApiErrorKind::NoSuchEntity,
                format!("IAM server certificate with name '{}' doesn't exist.", server_certificate_name).as_str(),
            ));
        }
    }
}

pub(crate) async fn tag_server_certificate(
    ctx: &OperationCtx, input: &TagServerCertificateRequest, db: &LocalDb,
) -> Result<TagServerCertificateOutput, OperationError> {
    input.validate("$")?;

    let mut tx = db.new_tx().await?;

    let server_certificate_id =
        find_id_by_name(tx.as_mut(), ctx.account_id, input.server_certificate_name().unwrap().trim()).await?;
    let mut server_certificate_tags = super::tag::prepare_for_db(input.tags(), server_certificate_id);

    db::Tags::ServerCertificate
        .save_all(&mut tx, &mut server_certificate_tags)
        .await?;
    let count = db::Tags::ServerCertificate
        .count(tx.as_mut(), server_certificate_id)
        .await?;
    if count > constants::tag::MAX_COUNT {
        return Err(OperationError::new(
            ApiErrorKind::LimitExceeded,
            format!("Cannot assign more than {} tags to IAM server certificate.", constants::tag::MAX_COUNT).as_str(),
        ));
    }

    let output = TagServerCertificateOutput::builder().build();

    tx.commit().await?;

    Ok(output)
}

pub(crate) async fn untag_server_certificate(
    ctx: &OperationCtx, input: &UntagServerCertificateRequest, db: &LocalDb,
) -> Result<UntagServerCertificateOutput, OperationError> {
    input.validate("$")?;

    let mut tx = db.new_tx().await?;

    let certificate_id =
        find_id_by_name(tx.as_mut(), ctx.account_id, input.server_certificate_name().unwrap().trim()).await?;

    db::Tags::ServerCertificate
        .delete_all(&mut tx, certificate_id, &input.tag_keys())
        .await?;

    let output = UntagServerCertificateOutput::builder().build();

    tx.commit().await?;

    Ok(output)
}

pub(crate) async fn list_server_certificate_tags(
    ctx: &OperationCtx, input: &ListServerCertificateTagsRequest, db: &LocalDb,
) -> Result<ListServerCertificateTagsOutput, OperationError> {
    input.validate("$")?;

    let mut connection = db.new_connection().await?;

    let server_certificate_id =
        find_id_by_name(connection.as_mut(), ctx.account_id, input.server_certificate_name().unwrap()).await?;

    let query = ListTagsQuery::new(input.max_items(), input.marker_type());
    let found_tags = db::Tags::ServerCertificate
        .list(connection.as_mut(), server_certificate_id, &query)
        .await?;

    let tags = super::common::convert_and_limit(&found_tags, query.limit);
    let marker = super::common::create_encoded_marker(&query, found_tags.len())?;

    let output = ListServerCertificateTagsOutput::builder()
        .set_tags(tags)
        .set_is_truncated(marker.as_ref().map(|_v| true))
        .set_marker(marker)
        .build()
        .unwrap();
    Ok(output)
}

pub(crate) async fn get_server_certificate(
    ctx: &OperationCtx, input: &GetServerCertificateRequest, db: &LocalDb,
) -> Result<GetServerCertificateOutput, OperationError> {
    input.validate("$")?;

    let output = GetServerCertificateOutput::builder().build();
    Ok(output)
}

pub(crate) async fn list_server_certificates(
    ctx: &OperationCtx, input: &ListServerCertificatesRequest, db: &LocalDb,
) -> Result<ListServerCertificatesOutput, OperationError> {
    input.validate("$")?;

    let output = ListServerCertificatesOutput::builder().build().unwrap();
    Ok(output)
}

pub(crate) async fn update_server_certificate(
    ctx: &OperationCtx, input: &UpdateServerCertificateRequest, db: &LocalDb,
) -> Result<UpdateServerCertificateOutput, OperationError> {
    input.validate("$")?;

    let output = UpdateServerCertificateOutput::builder().build();
    Ok(output)
}

pub(crate) async fn delete_server_certificate(
    ctx: &OperationCtx, input: &DeleteServerCertificateRequest, db: &LocalDb,
) -> Result<DeleteServerCertificateOutput, OperationError> {
    input.validate("$")?;

    let output = DeleteServerCertificateOutput::builder().build();
    Ok(output)
}
