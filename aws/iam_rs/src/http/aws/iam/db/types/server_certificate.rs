use aws_sdk_iam::types::ServerCertificateMetadata;
use aws_smithy_types::DateTime;
use sqlx::FromRow;

use crate::http::aws::iam::db::types::common::ListByPathQuery;
use crate::http::aws::iam::types::list_server_certificates::ListServerCertificatesRequest;

#[derive(Debug)]
pub(crate) struct InsertServerCertificate {
    pub(crate) id: Option<i64>,
    pub(crate) account_id: i64,
    pub(crate) arn: String,
    pub(crate) path: String,
    pub(crate) certificate_body: String,
    pub(crate) certificate_chain: Option<String>,
    pub(crate) server_certificate_name: String,
    pub(crate) server_certificate_id: String,
    pub(crate) upload_date: i64,
    pub(crate) expiration: i64,
}

#[derive(Debug)]
pub(crate) struct UpdateServerCertificateQuery {
    pub(crate) server_certificate_name: String,
    pub(crate) new_server_certificate_name: Option<String>,
    pub(crate) new_path: Option<String>,
}

#[derive(Debug, FromRow)]
pub(crate) struct SelectServerCertificate {
    pub(crate) id: i64,
    pub(crate) arn: String,
    pub(crate) path: String,
    pub(crate) certificate_body: String,
    pub(crate) certificate_chain: Option<String>,
    pub(crate) server_certificate_name: String,
    pub(crate) server_certificate_id: String,
    pub(crate) upload_date: i64,
    pub(crate) expiration: i64,
}

impl From<&SelectServerCertificate> for ServerCertificateMetadata {
    fn from(value: &SelectServerCertificate) -> Self {
        ServerCertificateMetadata::builder()
            .arn(&value.arn)
            .path(&value.path)
            .server_certificate_name(&value.server_certificate_name)
            .server_certificate_id(&value.server_certificate_id)
            .upload_date(DateTime::from_secs(value.upload_date))
            .expiration(DateTime::from_secs(value.expiration))
            .build()
            .unwrap()
    }
}

impl Into<ListByPathQuery> for &ListServerCertificatesRequest {
    fn into(self) -> ListByPathQuery {
        ListByPathQuery::new(self.path_prefix(), self.max_items(), self.marker_type())
    }
}
