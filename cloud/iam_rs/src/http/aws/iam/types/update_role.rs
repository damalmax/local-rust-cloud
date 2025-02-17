use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct UpdateRoleRequest {
    #[serde(rename = "RoleName")]
    pub(crate) role_name: Option<types::role_name_type::RoleNameType>,
    #[serde(rename = "Description")]
    pub(crate) description: Option<types::role_description_type::RoleDescriptionType>,
    #[serde(rename = "MaxSessionDuration")]
    pub(crate) max_session_duration: Option<types::role_max_session_duration_type::RoleMaxSessionDurationType>,
}

impl UpdateRoleRequest {
    pub(crate) fn role_name(&self) -> Option<&str> {
        self.role_name.as_deref()
    }
    pub(crate) fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    pub(crate) fn max_session_duration(&self) -> Option<&i32> {
        self.max_session_duration.as_deref()
    }
}

impl validators::NamedValidator for &UpdateRoleRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_required(self.role_name(), format!("{at}.{}", "RoleName").as_str())?;
        validators::validate_named(self.role_name.as_ref(), format!("{at}.{}", "RoleName").as_str())?;
        validators::validate_named(self.description.as_ref(), format!("{at}.{}", "Description").as_str())?;
        validators::validate_named(
            self.max_session_duration.as_ref(),
            format!("{at}.{}", "MaxSessionDuration").as_str(),
        )?;
        Ok(())
    }
}
