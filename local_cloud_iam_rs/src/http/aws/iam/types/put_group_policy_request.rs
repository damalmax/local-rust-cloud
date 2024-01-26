use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct PutGroupPolicyRequest {
    #[serde(rename = "PolicyName")]
    pub(crate) policy_name: Option<types::policy_name_type::PolicyNameType>,
    #[serde(rename = "GroupName")]
    pub(crate) group_name: Option<types::group_name_type::GroupNameType>,
    #[serde(rename = "PolicyDocument")]
    pub(crate) policy_document: Option<types::policy_document_type::PolicyDocumentType>,
}

impl PutGroupPolicyRequest {
    pub(crate) fn policy_name(&self) -> Option<&str> {
        self.policy_name.as_deref()
    }
    pub(crate) fn group_name(&self) -> Option<&str> {
        self.group_name.as_deref()
    }
    pub(crate) fn policy_document(&self) -> Option<&str> {
        self.policy_document.as_deref()
    }
}

impl local_cloud_validate::NamedValidator for &PutGroupPolicyRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_required(self.policy_name(), format!("{at}.{}", "PolicyName").as_str())?;
        local_cloud_validate::validate_named(self.policy_name.as_ref(), format!("{at}.{}", "PolicyName").as_str())?;
        local_cloud_validate::validate_required(self.group_name(), format!("{at}.{}", "GroupName").as_str())?;
        local_cloud_validate::validate_named(self.group_name.as_ref(), format!("{at}.{}", "GroupName").as_str())?;
        local_cloud_validate::validate_required(self.policy_document(), format!("{at}.{}", "PolicyDocument").as_str())?;
        local_cloud_validate::validate_named(
            self.policy_document.as_ref(),
            format!("{at}.{}", "PolicyDocument").as_str(),
        )?;
        Ok(())
    }
}
