use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct UploadServerCertificateRequest {
    #[serde(rename = "Path")]
    pub(crate) path: Option<types::path_type::PathType>,
    #[serde(rename = "CertificateBody")]
    pub(crate) certificate_body: Option<types::certificate_body_type::CertificateBodyType>,
    #[serde(rename = "ServerCertificateName")]
    pub(crate) server_certificate_name: Option<types::server_certificate_name_type::ServerCertificateNameType>,
    #[serde(rename = "PrivateKey")]
    pub(crate) private_key: Option<types::private_key_type::PrivateKeyType>,
    #[serde(rename = "Tags")]
    pub(crate) tags: Option<Vec<types::tag::Tag>>,
    #[serde(rename = "CertificateChain")]
    pub(crate) certificate_chain: Option<types::certificate_chain_type::CertificateChainType>,
}

impl UploadServerCertificateRequest {
    pub(crate) fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }
    pub(crate) fn certificate_body(&self) -> Option<&str> {
        self.certificate_body.as_deref()
    }
    pub(crate) fn server_certificate_name(&self) -> Option<&str> {
        self.server_certificate_name.as_deref()
    }
    pub(crate) fn private_key(&self) -> Option<&str> {
        self.private_key.as_deref()
    }
    pub(crate) fn tags(&self) -> Option<&[types::tag::Tag]> {
        self.tags.as_deref()
    }
    pub(crate) fn certificate_chain(&self) -> Option<&str> {
        self.certificate_chain.as_deref()
    }
}

impl local_cloud_validate::NamedValidator for &UploadServerCertificateRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_named(self.path.as_ref(), format!("{at}.{}", "Path").as_str())?;
        local_cloud_validate::validate_required(
            self.certificate_body(),
            format!("{at}.{}", "CertificateBody").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.certificate_body.as_ref(),
            format!("{at}.{}", "CertificateBody").as_str(),
        )?;
        local_cloud_validate::validate_required(
            self.server_certificate_name(),
            format!("{at}.{}", "ServerCertificateName").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.server_certificate_name.as_ref(),
            format!("{at}.{}", "ServerCertificateName").as_str(),
        )?;
        local_cloud_validate::validate_required(self.private_key(), format!("{at}.{}", "PrivateKey").as_str())?;
        local_cloud_validate::validate_named(self.private_key.as_ref(), format!("{at}.{}", "PrivateKey").as_str())?;
        local_cloud_validate::validate_array_size_min(self.tags(), 0usize, format!("{at}.{}", "Tags").as_str())?;
        local_cloud_validate::validate_array_size_max(self.tags(), 50usize, format!("{at}.{}", "Tags").as_str())?;
        if let Some(tags) = self.tags() {
            for (id, member) in tags.iter().enumerate() {
                local_cloud_validate::validate_named(Some(member), format!("{at}.{}.member.{id}", "Tags").as_str())?;
            }
        }
        local_cloud_validate::validate_named(
            self.certificate_chain.as_ref(),
            format!("{at}.{}", "CertificateChain").as_str(),
        )?;
        Ok(())
    }
}
