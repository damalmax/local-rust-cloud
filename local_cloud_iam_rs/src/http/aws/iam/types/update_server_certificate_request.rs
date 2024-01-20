use crate::http::aws::iam::types;
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct UpdateServerCertificateRequest {
    #[serde(rename = "ServerCertificateName")]
    pub(crate) server_certificate_name: Option<
        types::server_certificate_name_type::ServerCertificateNameType,
    >,
    #[serde(rename = "NewPath")]
    pub(crate) new_path: Option<types::path_type::PathType>,
    #[serde(rename = "NewServerCertificateName")]
    pub(crate) new_server_certificate_name: Option<
        types::server_certificate_name_type::ServerCertificateNameType,
    >,
}
impl UpdateServerCertificateRequest {
    pub(crate) fn server_certificate_name(&self) -> Option<&str> {
        self.server_certificate_name.as_deref()
    }
    pub(crate) fn new_path(&self) -> Option<&str> {
        self.new_path.as_deref()
    }
    pub(crate) fn new_server_certificate_name(&self) -> Option<&str> {
        self.new_server_certificate_name.as_deref()
    }
}
impl local_cloud_validate::NamedValidator for &UpdateServerCertificateRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_required(
            self.server_certificate_name(),
            format!("{at}.{}", "ServerCertificateName").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.server_certificate_name.as_ref(),
            format!("{at}.{}", "ServerCertificateName").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.new_path.as_ref(),
            format!("{at}.{}", "NewPath").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.new_server_certificate_name.as_ref(),
            format!("{at}.{}", "NewServerCertificateName").as_str(),
        )?;
        Ok(())
    }
}
