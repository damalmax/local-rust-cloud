use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct DeleteRolePolicyRequest {
    #[serde(rename = "RoleName")]
    pub(crate) role_name: Option<types::role_name_type::RoleNameType>,
    #[serde(rename = "PolicyName")]
    pub(crate) policy_name: Option<types::policy_name_type::PolicyNameType>,
}

impl DeleteRolePolicyRequest {
    pub(crate) fn role_name(&self) -> Option<&str> {
        self.role_name.as_deref()
    }
    pub(crate) fn policy_name(&self) -> Option<&str> {
        self.policy_name.as_deref()
    }
}

impl local_cloud_validate::NamedValidator for &DeleteRolePolicyRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_required(self.role_name(), format!("{at}.{}", "RoleName").as_str())?;
        local_cloud_validate::validate_named(self.role_name.as_ref(), format!("{at}.{}", "RoleName").as_str())?;
        local_cloud_validate::validate_required(self.policy_name(), format!("{at}.{}", "PolicyName").as_str())?;
        local_cloud_validate::validate_named(self.policy_name.as_ref(), format!("{at}.{}", "PolicyName").as_str())?;
        Ok(())
    }
}
