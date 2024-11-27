use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct DeleteServiceLinkedRoleRequest {
    #[serde(rename = "RoleName")]
    pub(crate) role_name: Option<types::role_name_type::RoleNameType>,
}

impl DeleteServiceLinkedRoleRequest {
    pub(crate) fn role_name(&self) -> Option<&str> {
        self.role_name.as_deref()
    }
}

impl validators::NamedValidator for &DeleteServiceLinkedRoleRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_required(self.role_name(), format!("{at}.{}", "RoleName").as_str())?;
        validators::validate_named(self.role_name.as_ref(), format!("{at}.{}", "RoleName").as_str())?;
        Ok(())
    }
}
