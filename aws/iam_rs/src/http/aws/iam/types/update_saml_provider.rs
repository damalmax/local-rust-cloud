use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct UpdateSamlProviderRequest {
    #[serde(rename = "SAMLMetadataDocument")]
    pub(crate) saml_metadata_document: Option<types::saml_metadata_document_type::SamlMetadataDocumentType>,
    #[serde(rename = "SAMLProviderArn")]
    pub(crate) saml_provider_arn: Option<types::arn_type::ArnType>,
}

impl UpdateSamlProviderRequest {
    pub(crate) fn saml_metadata_document(&self) -> Option<&str> {
        self.saml_metadata_document.as_deref()
    }
    pub(crate) fn saml_provider_arn(&self) -> Option<&str> {
        self.saml_provider_arn.as_deref()
    }
}

impl validators::NamedValidator for &UpdateSamlProviderRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_required(
            self.saml_metadata_document(),
            format!("{at}.{}", "SAMLMetadataDocument").as_str(),
        )?;
        validators::validate_named(
            self.saml_metadata_document.as_ref(),
            format!("{at}.{}", "SAMLMetadataDocument").as_str(),
        )?;
        validators::validate_required(
            self.saml_provider_arn(),
            format!("{at}.{}", "SAMLProviderArn").as_str(),
        )?;
        validators::validate_named(
            self.saml_provider_arn.as_ref(),
            format!("{at}.{}", "SAMLProviderArn").as_str(),
        )?;
        Ok(())
    }
}