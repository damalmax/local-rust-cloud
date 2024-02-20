use crate::http::aws::iam::db::types::signing_certificate_status_type::SigningCertificateStatusType;

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
