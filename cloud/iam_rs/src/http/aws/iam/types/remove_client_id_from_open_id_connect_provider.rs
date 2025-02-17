use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct RemoveClientIdFromOpenIdConnectProviderRequest {
    #[serde(rename = "ClientID")]
    pub(crate) client_id: Option<types::client_id_type::ClientIdType>,
    #[serde(rename = "OpenIDConnectProviderArn")]
    pub(crate) open_id_connect_provider_arn: Option<types::arn_type::ArnType>,
}

impl RemoveClientIdFromOpenIdConnectProviderRequest {
    pub(crate) fn client_id(&self) -> Option<&str> {
        self.client_id.as_deref()
    }
    pub(crate) fn open_id_connect_provider_arn(&self) -> Option<&str> {
        self.open_id_connect_provider_arn.as_deref()
    }
}

impl validators::NamedValidator for &RemoveClientIdFromOpenIdConnectProviderRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_required(self.client_id(), format!("{at}.{}", "ClientID").as_str())?;
        validators::validate_named(self.client_id.as_ref(), format!("{at}.{}", "ClientID").as_str())?;
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
