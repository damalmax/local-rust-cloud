use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct GetSamlProviderRequest {
    #[serde(rename = "SAMLProviderArn")]
    pub(crate) saml_provider_arn: Option<types::arn_type::ArnType>,
}

impl GetSamlProviderRequest {
    pub(crate) fn saml_provider_arn(&self) -> Option<&str> {
        self.saml_provider_arn.as_deref()
    }
}

impl validators::NamedValidator for &GetSamlProviderRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
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
