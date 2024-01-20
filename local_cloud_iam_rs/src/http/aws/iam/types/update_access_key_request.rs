use crate::http::aws::iam::types;
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct UpdateAccessKeyRequest {
    #[serde(rename = "UserName")]
    pub(crate) user_name: Option<types::existing_user_name_type::ExistingUserNameType>,
    #[serde(rename = "AccessKeyId")]
    pub(crate) access_key_id: Option<types::access_key_id_type::AccessKeyIdType>,
    #[serde(rename = "Status")]
    pub(crate) status: Option<types::status_type::StatusType>,
}
impl UpdateAccessKeyRequest {
    pub(crate) fn user_name(&self) -> Option<&str> {
        self.user_name.as_deref()
    }
    pub(crate) fn access_key_id(&self) -> Option<&str> {
        self.access_key_id.as_deref()
    }
    pub(crate) fn status(&self) -> Option<&types::status_type::StatusType> {
        self.status.as_ref()
    }
}
impl local_cloud_validate::NamedValidator for &UpdateAccessKeyRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_named(
            self.user_name.as_ref(),
            format!("{at}.{}", "UserName").as_str(),
        )?;
        local_cloud_validate::validate_required(
            self.access_key_id(),
            format!("{at}.{}", "AccessKeyId").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.access_key_id.as_ref(),
            format!("{at}.{}", "AccessKeyId").as_str(),
        )?;
        local_cloud_validate::validate_required(
            self.status(),
            format!("{at}.{}", "Status").as_str(),
        )?;
        Ok(())
    }
}
