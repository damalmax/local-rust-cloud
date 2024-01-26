use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct DeleteSamlProviderRequest {
    #[serde(rename = "SAMLProviderArn")]
    pub(crate) saml_provider_arn: Option<types::arn_type::ArnType>,
}

impl DeleteSamlProviderRequest {
    pub(crate) fn saml_provider_arn(&self) -> Option<&str> {
        self.saml_provider_arn.as_deref()
    }
}

impl local_cloud_validate::NamedValidator for &DeleteSamlProviderRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
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
