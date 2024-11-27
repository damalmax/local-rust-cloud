use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct CreateSamlProviderRequest {
    #[serde(rename = "SAMLMetadataDocument")]
    pub(crate) saml_metadata_document: Option<types::saml_metadata_document_type::SamlMetadataDocumentType>,
    #[serde(rename = "Name")]
    pub(crate) name: Option<types::saml_provider_name_type::SamlProviderNameType>,
    #[serde(rename = "Tags")]
    pub(crate) tags: Option<Vec<types::tag::Tag>>,
}

impl CreateSamlProviderRequest {
    pub(crate) fn saml_metadata_document(&self) -> Option<&str> {
        self.saml_metadata_document.as_deref()
    }
    pub(crate) fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
    pub(crate) fn tags(&self) -> Option<&[types::tag::Tag]> {
        self.tags.as_deref()
    }
}

impl validators::NamedValidator for &CreateSamlProviderRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_required(
            self.saml_metadata_document(),
            format!("{at}.{}", "SAMLMetadataDocument").as_str(),
        )?;
        validators::validate_named(
            self.saml_metadata_document.as_ref(),
            format!("{at}.{}", "SAMLMetadataDocument").as_str(),
        )?;
        validators::validate_required(self.name(), format!("{at}.{}", "Name").as_str())?;
        validators::validate_named(self.name.as_ref(), format!("{at}.{}", "Name").as_str())?;
        validators::validate_array_size_min(self.tags(), 0usize, format!("{at}.{}", "Tags").as_str())?;
        validators::validate_array_size_max(self.tags(), 50usize, format!("{at}.{}", "Tags").as_str())?;
        if let Some(tags) = self.tags() {
            for (id, member) in tags.iter().enumerate() {
                validators::validate_named(Some(member), format!("{at}.{}.member.{id}", "Tags").as_str())?;
            }
        }
        Ok(())
    }
}
