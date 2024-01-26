use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct GetAccessKeyLastUsedRequest {
    #[serde(rename = "AccessKeyId")]
    pub(crate) access_key_id: Option<types::access_key_id_type::AccessKeyIdType>,
}

impl GetAccessKeyLastUsedRequest {
    pub(crate) fn access_key_id(&self) -> Option<&str> {
        self.access_key_id.as_deref()
    }
}

impl local_cloud_validate::NamedValidator for &GetAccessKeyLastUsedRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_required(self.access_key_id(), format!("{at}.{}", "AccessKeyId").as_str())?;
        local_cloud_validate::validate_named(self.access_key_id.as_ref(), format!("{at}.{}", "AccessKeyId").as_str())?;
        Ok(())
    }
}
