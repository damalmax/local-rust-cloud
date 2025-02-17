use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct SetSecurityTokenServicePreferencesRequest {
    #[serde(rename = "GlobalEndpointTokenVersion")]
    pub(crate) global_endpoint_token_version: Option<types::global_endpoint_token_version::GlobalEndpointTokenVersion>,
}

impl SetSecurityTokenServicePreferencesRequest {
    pub(crate) fn global_endpoint_token_version(
        &self,
    ) -> Option<&types::global_endpoint_token_version::GlobalEndpointTokenVersion> {
        self.global_endpoint_token_version.as_ref()
    }
}

impl validators::NamedValidator for &SetSecurityTokenServicePreferencesRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_required(
            self.global_endpoint_token_version(),
            format!("{at}.{}", "GlobalEndpointTokenVersion").as_str(),
        )?;
        Ok(())
    }
}
