use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct UpdateAssumeRolePolicyRequest {
    #[serde(rename = "PolicyDocument")]
    pub(crate) policy_document: Option<types::policy_document_type::PolicyDocumentType>,
    #[serde(rename = "RoleName")]
    pub(crate) role_name: Option<types::role_name_type::RoleNameType>,
}

impl UpdateAssumeRolePolicyRequest {
    pub(crate) fn policy_document(&self) -> Option<&str> {
        self.policy_document.as_deref()
    }
    pub(crate) fn role_name(&self) -> Option<&str> {
        self.role_name.as_deref()
    }
}

impl local_cloud_validate::NamedValidator for &UpdateAssumeRolePolicyRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_required(self.policy_document(), format!("{at}.{}", "PolicyDocument").as_str())?;
        local_cloud_validate::validate_named(
            self.policy_document.as_ref(),
            format!("{at}.{}", "PolicyDocument").as_str(),
        )?;
        local_cloud_validate::validate_required(self.role_name(), format!("{at}.{}", "RoleName").as_str())?;
        local_cloud_validate::validate_named(self.role_name.as_ref(), format!("{at}.{}", "RoleName").as_str())?;
        Ok(())
    }
}
