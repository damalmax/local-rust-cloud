use crate::http::aws::iam::db::types::signing_certificate_status_type::SigningCertificateStatusType;
use aws_sdk_iam::types::{SigningCertificate, StatusType};
use aws_smithy_types::DateTime;
use sqlx::FromRow;

#[derive(Debug)]
pub(crate) struct InsertSigningCertificate {
    pub(crate) id: Option<i64>,
    pub(crate) account_id: i64,
    pub(crate) certificate_id: String,
    pub(crate) certificate_body: String,
    pub(crate) status: SigningCertificateStatusType,
    pub(crate) upload_date: i64,
    pub(crate) user_id: i64,
}

#[derive(Debug, FromRow)]
pub(crate) struct SelectSigningCertificate {
    pub(crate) id: i64,
    pub(crate) certificate_id: String,
    pub(crate) certificate_body: String,
    pub(crate) status: SigningCertificateStatusType,
    pub(crate) upload_date: i64,
    pub(crate) user_id: i64,
    pub(crate) user_name: String,
}

impl SelectSigningCertificate {
    fn status(&self) -> &SigningCertificateStatusType {
        &self.status
    }
}

impl From<&SelectSigningCertificate> for SigningCertificate {
    fn from(value: &SelectSigningCertificate) -> Self {
        let status: StatusType = value.status().into();
        SigningCertificate::builder()
            .user_name(&value.user_name)
            .status(status)
            .upload_date(DateTime::from_secs(value.upload_date))
            .certificate_id(&value.certificate_id)
            .certificate_body(&value.certificate_body)
            .build()
            .unwrap()
    }
}

#[derive(Debug)]
pub(crate) struct UpdateSigningCertificateQuery {
    pub(crate) certificate_id: String,
    pub(crate) status: SigningCertificateStatusType,
    pub(crate) user_id: i64,
}
