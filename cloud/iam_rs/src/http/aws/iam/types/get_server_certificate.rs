use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct GetServerCertificateRequest {
    #[serde(rename = "ServerCertificateName")]
    pub(crate) server_certificate_name: Option<types::server_certificate_name_type::ServerCertificateNameType>,
}

impl GetServerCertificateRequest {
    pub(crate) fn server_certificate_name(&self) -> Option<&str> {
        self.server_certificate_name.as_deref()
    }
}

impl validators::NamedValidator for &GetServerCertificateRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_required(
            self.server_certificate_name(),
            format!("{at}.{}", "ServerCertificateName").as_str(),
        )?;
        validators::validate_named(
            self.server_certificate_name.as_ref(),
            format!("{at}.{}", "ServerCertificateName").as_str(),
        )?;
        Ok(())
    }
}
