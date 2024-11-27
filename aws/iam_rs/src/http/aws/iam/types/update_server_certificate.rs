use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct UpdateServerCertificateRequest {
    #[serde(rename = "NewServerCertificateName")]
    pub(crate) new_server_certificate_name: Option<types::server_certificate_name_type::ServerCertificateNameType>,
    #[serde(rename = "ServerCertificateName")]
    pub(crate) server_certificate_name: Option<types::server_certificate_name_type::ServerCertificateNameType>,
    #[serde(rename = "NewPath")]
    pub(crate) new_path: Option<types::path_type::PathType>,
}

impl UpdateServerCertificateRequest {
    pub(crate) fn new_server_certificate_name(&self) -> Option<&str> {
        self.new_server_certificate_name.as_deref()
    }
    pub(crate) fn server_certificate_name(&self) -> Option<&str> {
        self.server_certificate_name.as_deref()
    }
    pub(crate) fn new_path(&self) -> Option<&str> {
        self.new_path.as_deref()
    }
}

impl validators::NamedValidator for &UpdateServerCertificateRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_named(
            self.new_server_certificate_name.as_ref(),
            format!("{at}.{}", "NewServerCertificateName").as_str(),
        )?;
        validators::validate_required(
            self.server_certificate_name(),
            format!("{at}.{}", "ServerCertificateName").as_str(),
        )?;
        validators::validate_named(
            self.server_certificate_name.as_ref(),
            format!("{at}.{}", "ServerCertificateName").as_str(),
        )?;
        validators::validate_named(self.new_path.as_ref(), format!("{at}.{}", "NewPath").as_str())?;
        Ok(())
    }
}
