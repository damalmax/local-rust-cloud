use crate::http::aws::iam::types;
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct UpdateOpenIdConnectProviderThumbprintRequest {
    #[serde(rename = "OpenIDConnectProviderArn")]
    pub(crate) open_id_connect_provider_arn: Option<types::arn_type::ArnType>,
    #[serde(rename = "ThumbprintList")]
    pub(crate) thumbprint_list: Option<Vec<types::thumbprint_type::ThumbprintType>>,
}
impl UpdateOpenIdConnectProviderThumbprintRequest {
    pub(crate) fn open_id_connect_provider_arn(&self) -> Option<&str> {
        self.open_id_connect_provider_arn.as_deref()
    }
    pub(crate) fn thumbprint_list(
        &self,
    ) -> Option<&[types::thumbprint_type::ThumbprintType]> {
        self.thumbprint_list.as_deref()
    }
}
impl local_cloud_validate::NamedValidator
for &UpdateOpenIdConnectProviderThumbprintRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_required(
            self.open_id_connect_provider_arn(),
            format!("{at}.{}", "OpenIDConnectProviderArn").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.open_id_connect_provider_arn.as_ref(),
            format!("{at}.{}", "OpenIDConnectProviderArn").as_str(),
        )?;
        local_cloud_validate::validate_required(
            self.thumbprint_list(),
            format!("{at}.{}", "ThumbprintList").as_str(),
        )?;
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
