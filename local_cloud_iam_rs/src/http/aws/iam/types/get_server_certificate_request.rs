use crate::http::aws::iam::types;
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct GetServerCertificateRequest {
    #[serde(rename = "ServerCertificateName")]
    pub(crate) server_certificate_name: Option<
        types::server_certificate_name_type::ServerCertificateNameType,
    >,
}
impl GetServerCertificateRequest {
    pub(crate) fn server_certificate_name(&self) -> Option<&str> {
        self.server_certificate_name.as_deref()
    }
}
impl local_cloud_validate::NamedValidator for &GetServerCertificateRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_required(
            self.server_certificate_name(),
            format!("{at}.{}", "ServerCertificateName").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.server_certificate_name.as_ref(),
            format!("{at}.{}", "ServerCertificateName").as_str(),
        )?;
        Ok(())
    }
}
