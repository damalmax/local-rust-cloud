use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct UntagServerCertificateRequest {
    #[serde(rename = "TagKeys")]
    pub(crate) tag_keys: Option<Vec<types::tag_key_type::TagKeyType>>,
    #[serde(rename = "ServerCertificateName")]
    pub(crate) server_certificate_name: Option<types::server_certificate_name_type::ServerCertificateNameType>,
}

impl UntagServerCertificateRequest {
    pub(crate) fn tag_keys(&self) -> Option<&[types::tag_key_type::TagKeyType]> {
        self.tag_keys.as_deref()
    }
    pub(crate) fn server_certificate_name(&self) -> Option<&str> {
        self.server_certificate_name.as_deref()
    }
}

impl local_cloud_validate::NamedValidator for &UntagServerCertificateRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_required(self.tag_keys(), format!("{at}.{}", "TagKeys").as_str())?;
        local_cloud_validate::validate_array_size_min(self.tag_keys(), 0usize, format!("{at}.{}", "TagKeys").as_str())?;
        local_cloud_validate::validate_array_size_max(
            self.tag_keys(),
            50usize,
            format!("{at}.{}", "TagKeys").as_str(),
        )?;
        if let Some(tag_keys) = self.tag_keys() {
            for (id, member) in tag_keys.iter().enumerate() {
                local_cloud_validate::validate_named(Some(member), format!("{at}.{}.member.{id}", "TagKeys").as_str())?;
            }
        }
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
