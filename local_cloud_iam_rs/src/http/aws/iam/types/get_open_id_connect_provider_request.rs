use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct GetOpenIdConnectProviderRequest {
    #[serde(rename = "OpenIDConnectProviderArn")]
    pub(crate) open_id_connect_provider_arn: Option<types::arn_type::ArnType>,
}

impl GetOpenIdConnectProviderRequest {
    pub(crate) fn open_id_connect_provider_arn(&self) -> Option<&str> {
        self.open_id_connect_provider_arn.as_deref()
    }
}

impl local_cloud_validate::NamedValidator for &GetOpenIdConnectProviderRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_required(
            self.open_id_connect_provider_arn(),
            format!("{at}.{}", "OpenIDConnectProviderArn").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.open_id_connect_provider_arn.as_ref(),
            format!("{at}.{}", "OpenIDConnectProviderArn").as_str(),
        )?;
        Ok(())
    }
}
