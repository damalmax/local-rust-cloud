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
