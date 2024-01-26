use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct UploadSigningCertificateRequest {
    #[serde(rename = "UserName")]
    pub(crate) user_name: Option<types::existing_user_name_type::ExistingUserNameType>,
    #[serde(rename = "CertificateBody")]
    pub(crate) certificate_body: Option<types::certificate_body_type::CertificateBodyType>,
}

impl UploadSigningCertificateRequest {
    pub(crate) fn user_name(&self) -> Option<&str> {
        self.user_name.as_deref()
    }
    pub(crate) fn certificate_body(&self) -> Option<&str> {
        self.certificate_body.as_deref()
    }
}

impl local_cloud_validate::NamedValidator for &UploadSigningCertificateRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_named(self.user_name.as_ref(), format!("{at}.{}", "UserName").as_str())?;
        local_cloud_validate::validate_required(
            self.certificate_body(),
            format!("{at}.{}", "CertificateBody").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.certificate_body.as_ref(),
            format!("{at}.{}", "CertificateBody").as_str(),
        )?;
        Ok(())
    }
}
