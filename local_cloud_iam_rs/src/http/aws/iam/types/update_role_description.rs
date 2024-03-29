use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct UpdateRoleDescriptionRequest {
    #[serde(rename = "RoleName")]
    pub(crate) role_name: Option<types::role_name_type::RoleNameType>,
    #[serde(rename = "Description")]
    pub(crate) description: Option<types::role_description_type::RoleDescriptionType>,
}

impl UpdateRoleDescriptionRequest {
    pub(crate) fn role_name(&self) -> Option<&str> {
        self.role_name.as_deref()
    }
    pub(crate) fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

impl local_cloud_validate::NamedValidator for &UpdateRoleDescriptionRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_required(self.role_name(), format!("{at}.{}", "RoleName").as_str())?;
        local_cloud_validate::validate_named(self.role_name.as_ref(), format!("{at}.{}", "RoleName").as_str())?;
        local_cloud_validate::validate_required(self.description(), format!("{at}.{}", "Description").as_str())?;
        local_cloud_validate::validate_named(self.description.as_ref(), format!("{at}.{}", "Description").as_str())?;
        Ok(())
    }
}
