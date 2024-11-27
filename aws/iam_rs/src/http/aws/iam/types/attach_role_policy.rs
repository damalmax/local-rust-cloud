use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct AttachRolePolicyRequest {
    #[serde(rename = "RoleName")]
    pub(crate) role_name: Option<types::role_name_type::RoleNameType>,
    #[serde(rename = "PolicyArn")]
    pub(crate) policy_arn: Option<types::arn_type::ArnType>,
}

impl AttachRolePolicyRequest {
    pub(crate) fn role_name(&self) -> Option<&str> {
        self.role_name.as_deref()
    }
    pub(crate) fn policy_arn(&self) -> Option<&str> {
        self.policy_arn.as_deref()
    }
}

impl validators::NamedValidator for &AttachRolePolicyRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_required(self.role_name(), format!("{at}.{}", "RoleName").as_str())?;
        validators::validate_named(self.role_name.as_ref(), format!("{at}.{}", "RoleName").as_str())?;
        validators::validate_required(self.policy_arn(), format!("{at}.{}", "PolicyArn").as_str())?;
        validators::validate_named(self.policy_arn.as_ref(), format!("{at}.{}", "PolicyArn").as_str())?;
        Ok(())
    }
}
