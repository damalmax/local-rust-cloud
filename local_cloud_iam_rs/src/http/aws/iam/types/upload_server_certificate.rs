use local_cloud_validate::{validate_array_size_max, validate_array_size_min, validate_named, validate_required};
use types::certificate_body_type::CertificateBodyType;
use types::certificate_chain_type::CertificateChainType;
use types::private_key_type::PrivateKeyType;
use types::server_certificate_name_type::ServerCertificateNameType;

use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct UploadServerCertificateRequest {
    #[serde(rename = "Path")]
    pub(crate) path: Option<types::path_type::PathType>,
    #[serde(rename = "CertificateBody")]
    pub(crate) certificate_body: Option<CertificateBodyType>,
    #[serde(rename = "ServerCertificateName")]
    pub(crate) server_certificate_name: Option<ServerCertificateNameType>,
    #[serde(rename = "PrivateKey")]
    pub(crate) private_key: Option<PrivateKeyType>,
    #[serde(rename = "Tags")]
    pub(crate) tags: Option<Vec<types::tag::Tag>>,
    #[serde(rename = "CertificateChain")]
    pub(crate) certificate_chain: Option<CertificateChainType>,
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
        validate_named(self.path.as_ref(), format!("{at}.{}", "Path").as_str())?;
        validate_required(self.certificate_body(), format!("{at}.{}", "CertificateBody").as_str())?;
        validate_named(self.certificate_body.as_ref(), format!("{at}.{}", "CertificateBody").as_str())?;
        validate_required(self.server_certificate_name(), format!("{at}.{}", "ServerCertificateName").as_str())?;
        validate_named(self.server_certificate_name.as_ref(), format!("{at}.{}", "ServerCertificateName").as_str())?;
        validate_required(self.private_key(), format!("{at}.{}", "PrivateKey").as_str())?;
        validate_named(self.private_key.as_ref(), format!("{at}.{}", "PrivateKey").as_str())?;
        validate_array_size_min(self.tags(), 0usize, format!("{at}.{}", "Tags").as_str())?;
        validate_array_size_max(self.tags(), 50usize, format!("{at}.{}", "Tags").as_str())?;
        if let Some(tags) = self.tags() {
            for (id, member) in tags.iter().enumerate() {
                validate_named(Some(member), format!("{at}.{}.member.{id}", "Tags").as_str())?;
            }
        }
        validate_named(self.certificate_chain.as_ref(), format!("{at}.{}", "CertificateChain").as_str())?;
        Ok(())
    }
}
