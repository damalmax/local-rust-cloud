use crate::http::aws::iam::types;
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct CreateServiceLinkedRoleRequest {
    #[serde(rename = "Description")]
    pub(crate) description: Option<types::role_description_type::RoleDescriptionType>,
    #[serde(rename = "CustomSuffix")]
    pub(crate) custom_suffix: Option<types::custom_suffix_type::CustomSuffixType>,
    #[serde(rename = "AWSServiceName")]
    pub(crate) aws_service_name: Option<types::group_name_type::GroupNameType>,
}
impl CreateServiceLinkedRoleRequest {
    pub(crate) fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    pub(crate) fn custom_suffix(&self) -> Option<&str> {
        self.custom_suffix.as_deref()
    }
    pub(crate) fn aws_service_name(&self) -> Option<&str> {
        self.aws_service_name.as_deref()
    }
}
impl local_cloud_validate::NamedValidator for &CreateServiceLinkedRoleRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_named(self.description.as_ref(), format!("{at}.{}", "Description").as_str())?;
        local_cloud_validate::validate_named(self.custom_suffix.as_ref(), format!("{at}.{}", "CustomSuffix").as_str())?;
        local_cloud_validate::validate_required(
            self.aws_service_name(),
            format!("{at}.{}", "AWSServiceName").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.aws_service_name.as_ref(),
            format!("{at}.{}", "AWSServiceName").as_str(),
        )?;
        Ok(())
    }
}
