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

impl validators::NamedValidator for &GetAccessKeyLastUsedRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_required(self.access_key_id(), format!("{at}.{}", "AccessKeyId").as_str())?;
        validators::validate_named(self.access_key_id.as_ref(), format!("{at}.{}", "AccessKeyId").as_str())?;
        Ok(())
    }
}
