use crate::http::aws::iam::types;
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct UntagOpenIdConnectProviderRequest {
    #[serde(rename = "TagKeys")]
    pub(crate) tag_keys: Option<Vec<types::tag_key_type::TagKeyType>>,
    #[serde(rename = "OpenIDConnectProviderArn")]
    pub(crate) open_id_connect_provider_arn: Option<types::arn_type::ArnType>,
}
impl UntagOpenIdConnectProviderRequest {
    pub(crate) fn tag_keys(&self) -> Option<&[types::tag_key_type::TagKeyType]> {
        self.tag_keys.as_deref()
    }
    pub(crate) fn open_id_connect_provider_arn(&self) -> Option<&str> {
        self.open_id_connect_provider_arn.as_deref()
    }
}
impl local_cloud_validate::NamedValidator for &UntagOpenIdConnectProviderRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_required(self.tag_keys(), format!("{at}.{}", "TagKeys").as_str())?;
        local_cloud_validate::validate_array_size_min(self.tag_keys(), 0usize, format!("{at}.{}", "TagKeys").as_str())?;
        local_cloud_validate::validate_array_size_max(
            self.tag_keys(),
            50usize,
            format!("{at}.{}", "TagKeys").as_str(),
        )?;
        if let Some(tag_keys) = self.tag_keys() {
            for (id, member) in tag_keys.iter().enumerate() {
                local_cloud_validate::validate_named(Some(member), format!("{at}.{}.member.{id}", "TagKeys").as_str())?;
            }
        }
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
