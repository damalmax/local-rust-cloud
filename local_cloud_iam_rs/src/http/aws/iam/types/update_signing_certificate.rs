use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct UpdateSigningCertificateRequest {
    #[serde(rename = "CertificateId")]
    pub(crate) certificate_id: Option<types::certificate_id_type::CertificateIdType>,
    #[serde(rename = "UserName")]
    pub(crate) user_name: Option<types::existing_user_name_type::ExistingUserNameType>,
    #[serde(rename = "Status")]
    pub(crate) status: Option<types::status_type::StatusType>,
}

impl UpdateSigningCertificateRequest {
    pub(crate) fn certificate_id(&self) -> Option<&str> {
        self.certificate_id.as_deref()
    }
    pub(crate) fn user_name(&self) -> Option<&str> {
        self.user_name.as_deref()
    }
    pub(crate) fn status(&self) -> Option<&types::status_type::StatusType> {
        self.status.as_ref()
    }
}

impl local_cloud_validate::NamedValidator for &UpdateSigningCertificateRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_required(self.certificate_id(), format!("{at}.{}", "CertificateId").as_str())?;
        local_cloud_validate::validate_named(
            self.certificate_id.as_ref(),
            format!("{at}.{}", "CertificateId").as_str(),
        )?;
        local_cloud_validate::validate_named(self.user_name.as_ref(), format!("{at}.{}", "UserName").as_str())?;
        local_cloud_validate::validate_required(self.status(), format!("{at}.{}", "Status").as_str())?;
        Ok(())
    }
}
