use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct AddRoleToInstanceProfileRequest {
    #[serde(rename = "RoleName")]
    pub(crate) role_name: Option<types::role_name_type::RoleNameType>,
    #[serde(rename = "InstanceProfileName")]
    pub(crate) instance_profile_name: Option<types::instance_profile_name_type::InstanceProfileNameType>,
}

impl AddRoleToInstanceProfileRequest {
    pub(crate) fn role_name(&self) -> Option<&str> {
        self.role_name.as_deref()
    }
    pub(crate) fn instance_profile_name(&self) -> Option<&str> {
        self.instance_profile_name.as_deref()
    }
}

impl validators::NamedValidator for &AddRoleToInstanceProfileRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_required(self.role_name(), format!("{at}.{}", "RoleName").as_str())?;
        validators::validate_named(self.role_name.as_ref(), format!("{at}.{}", "RoleName").as_str())?;
        validators::validate_required(
            self.instance_profile_name(),
            format!("{at}.{}", "InstanceProfileName").as_str(),
        )?;
        validators::validate_named(
            self.instance_profile_name.as_ref(),
            format!("{at}.{}", "InstanceProfileName").as_str(),
        )?;
        Ok(())
    }
}
