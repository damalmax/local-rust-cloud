use local_cloud_validate::{validate_named, validate_required};

use crate::http::aws::iam::types;
use crate::http::aws::iam::types::certificate_body_type::CertificateBodyType;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct UploadSigningCertificateRequest {
    #[serde(rename = "UserName")]
    pub(crate) user_name: Option<types::existing_user_name_type::ExistingUserNameType>,
    #[serde(rename = "CertificateBody")]
    pub(crate) certificate_body: Option<CertificateBodyType>,
}

impl UploadSigningCertificateRequest {
    pub(crate) fn user_name(&self) -> Option<&str> {
        self.user_name.as_deref()
    }
    pub(crate) fn certificate_body(&self) -> Option<&str> {
        self.certificate_body.as_deref()
    }
    pub(crate) fn certificate_body_type(&self) -> Option<&CertificateBodyType> {
        self.certificate_body.as_ref()
    }
}

impl local_cloud_validate::NamedValidator for &UploadSigningCertificateRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        validate_named(self.user_name.as_ref(), format!("{at}.{}", "UserName").as_str())?;
        validate_required(self.certificate_body(), format!("{at}.{}", "CertificateBody").as_str())?;
        validate_named(self.certificate_body.as_ref(), format!("{at}.{}", "CertificateBody").as_str())?;
        Ok(())
    }
}
