use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct DeleteOpenIdConnectProviderRequest {
    #[serde(rename = "OpenIDConnectProviderArn")]
    pub(crate) open_id_connect_provider_arn: Option<types::arn_type::ArnType>,
}

impl DeleteOpenIdConnectProviderRequest {
    pub(crate) fn open_id_connect_provider_arn(&self) -> Option<&str> {
        self.open_id_connect_provider_arn.as_deref()
    }
}

impl validators::NamedValidator for &DeleteOpenIdConnectProviderRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_required(
            self.open_id_connect_provider_arn(),
            format!("{at}.{}", "OpenIDConnectProviderArn").as_str(),
        )?;
        validators::validate_named(
            self.open_id_connect_provider_arn.as_ref(),
            format!("{at}.{}", "OpenIDConnectProviderArn").as_str(),
        )?;
        Ok(())
    }
}
