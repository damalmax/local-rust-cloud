use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct CreateOpenIdConnectProviderRequest {
    #[serde(rename = "ClientIDList")]
    pub(crate) client_id_list: Option<Vec<types::client_id_type::ClientIdType>>,
    #[serde(rename = "Url")]
    pub(crate) url: Option<types::open_id_connect_provider_url_type::OpenIdConnectProviderUrlType>,
    #[serde(rename = "Tags")]
    pub(crate) tags: Option<Vec<types::tag::Tag>>,
    #[serde(rename = "ThumbprintList")]
    pub(crate) thumbprint_list: Option<Vec<types::thumbprint_type::ThumbprintType>>,
}

impl CreateOpenIdConnectProviderRequest {
    pub(crate) fn client_id_list(&self) -> Option<&[types::client_id_type::ClientIdType]> {
        self.client_id_list.as_deref()
    }
    pub(crate) fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }
    pub(crate) fn tags(&self) -> Option<&[types::tag::Tag]> {
        self.tags.as_deref()
    }
    pub(crate) fn thumbprint_list(&self) -> Option<&[types::thumbprint_type::ThumbprintType]> {
        self.thumbprint_list.as_deref()
    }
}

impl local_cloud_validate::NamedValidator for &CreateOpenIdConnectProviderRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        if let Some(client_id_list) = self.client_id_list() {
            for (id, member) in client_id_list.iter().enumerate() {
                local_cloud_validate::validate_named(
                    Some(member),
                    format!("{at}.{}.member.{id}", "ClientIDList").as_str(),
                )?;
            }
        }
        local_cloud_validate::validate_required(self.url(), format!("{at}.{}", "Url").as_str())?;
        local_cloud_validate::validate_named(self.url.as_ref(), format!("{at}.{}", "Url").as_str())?;
        local_cloud_validate::validate_array_size_min(self.tags(), 0usize, format!("{at}.{}", "Tags").as_str())?;
        local_cloud_validate::validate_array_size_max(self.tags(), 50usize, format!("{at}.{}", "Tags").as_str())?;
        if let Some(tags) = self.tags() {
            for (id, member) in tags.iter().enumerate() {
                local_cloud_validate::validate_named(Some(member), format!("{at}.{}.member.{id}", "Tags").as_str())?;
            }
        }
        local_cloud_validate::validate_required(self.thumbprint_list(), format!("{at}.{}", "ThumbprintList").as_str())?;
        if let Some(thumbprint_list) = self.thumbprint_list() {
            for (id, member) in thumbprint_list.iter().enumerate() {
                local_cloud_validate::validate_named(
                    Some(member),
                    format!("{at}.{}.member.{id}", "ThumbprintList").as_str(),
                )?;
            }
        }
        Ok(())
    }
}
