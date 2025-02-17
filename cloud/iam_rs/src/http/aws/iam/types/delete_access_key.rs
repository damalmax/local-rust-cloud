use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct DeleteAccessKeyRequest {
    #[serde(rename = "UserName")]
    pub(crate) user_name: Option<types::existing_user_name_type::ExistingUserNameType>,
    #[serde(rename = "AccessKeyId")]
    pub(crate) access_key_id: Option<types::access_key_id_type::AccessKeyIdType>,
}

impl DeleteAccessKeyRequest {
    pub(crate) fn user_name(&self) -> Option<&str> {
        self.user_name.as_deref()
    }
    pub(crate) fn access_key_id(&self) -> Option<&str> {
        self.access_key_id.as_deref()
    }
}

impl validators::NamedValidator for &DeleteAccessKeyRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_named(self.user_name.as_ref(), format!("{at}.{}", "UserName").as_str())?;
        validators::validate_required(self.access_key_id(), format!("{at}.{}", "AccessKeyId").as_str())?;
        validators::validate_named(self.access_key_id.as_ref(), format!("{at}.{}", "AccessKeyId").as_str())?;
        Ok(())
    }
}
