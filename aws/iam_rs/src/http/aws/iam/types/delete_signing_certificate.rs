use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct DeleteSigningCertificateRequest {
    #[serde(rename = "UserName")]
    pub(crate) user_name: Option<types::existing_user_name_type::ExistingUserNameType>,
    #[serde(rename = "CertificateId")]
    pub(crate) certificate_id: Option<types::certificate_id_type::CertificateIdType>,
}

impl DeleteSigningCertificateRequest {
    pub(crate) fn user_name(&self) -> Option<&str> {
        self.user_name.as_deref()
    }
    pub(crate) fn certificate_id(&self) -> Option<&str> {
        self.certificate_id.as_deref()
    }
}

impl validators::NamedValidator for &DeleteSigningCertificateRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_named(self.user_name.as_ref(), format!("{at}.{}", "UserName").as_str())?;
        validators::validate_required(self.certificate_id(), format!("{at}.{}", "CertificateId").as_str())?;
        validators::validate_named(
            self.certificate_id.as_ref(),
            format!("{at}.{}", "CertificateId").as_str(),
        )?;
        Ok(())
    }
}
