use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct TagSamlProviderRequest {
    #[serde(rename = "Tags")]
    pub(crate) tags: Option<Vec<types::tag::Tag>>,
    #[serde(rename = "SAMLProviderArn")]
    pub(crate) saml_provider_arn: Option<types::arn_type::ArnType>,
}

impl TagSamlProviderRequest {
    pub(crate) fn tags(&self) -> Option<&[types::tag::Tag]> {
        self.tags.as_deref()
    }
    pub(crate) fn saml_provider_arn(&self) -> Option<&str> {
        self.saml_provider_arn.as_deref()
    }
}

impl local_cloud_validate::NamedValidator for &TagSamlProviderRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_required(self.tags(), format!("{at}.{}", "Tags").as_str())?;
        local_cloud_validate::validate_array_size_min(self.tags(), 0usize, format!("{at}.{}", "Tags").as_str())?;
        local_cloud_validate::validate_array_size_max(self.tags(), 50usize, format!("{at}.{}", "Tags").as_str())?;
        if let Some(tags) = self.tags() {
            for (id, member) in tags.iter().enumerate() {
                local_cloud_validate::validate_named(Some(member), format!("{at}.{}.member.{id}", "Tags").as_str())?;
            }
        }
        local_cloud_validate::validate_required(
            self.saml_provider_arn(),
            format!("{at}.{}", "SAMLProviderArn").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.saml_provider_arn.as_ref(),
            format!("{at}.{}", "SAMLProviderArn").as_str(),
        )?;
        Ok(())
    }
}
