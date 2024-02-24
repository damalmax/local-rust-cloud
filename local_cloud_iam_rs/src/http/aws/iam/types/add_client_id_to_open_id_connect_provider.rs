use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct AddClientIdToOpenIdConnectProviderRequest {
    #[serde(rename = "OpenIDConnectProviderArn")]
    pub(crate) open_id_connect_provider_arn: Option<types::arn_type::ArnType>,
    #[serde(rename = "ClientID")]
    pub(crate) client_id: Option<types::client_id_type::ClientIdType>,
}

impl AddClientIdToOpenIdConnectProviderRequest {
    pub(crate) fn open_id_connect_provider_arn(&self) -> Option<&str> {
        self.open_id_connect_provider_arn.as_deref()
    }
    pub(crate) fn client_id(&self) -> Option<&str> {
        self.client_id.as_deref()
    }
}

impl local_cloud_validate::NamedValidator for &AddClientIdToOpenIdConnectProviderRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_required(
            self.open_id_connect_provider_arn(),
            format!("{at}.{}", "OpenIDConnectProviderArn").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.open_id_connect_provider_arn.as_ref(),
            format!("{at}.{}", "OpenIDConnectProviderArn").as_str(),
        )?;
        local_cloud_validate::validate_required(self.client_id(), format!("{at}.{}", "ClientID").as_str())?;
        local_cloud_validate::validate_named(self.client_id.as_ref(), format!("{at}.{}", "ClientID").as_str())?;
        Ok(())
    }
}
