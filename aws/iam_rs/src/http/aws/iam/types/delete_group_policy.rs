use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct DeleteGroupPolicyRequest {
    #[serde(rename = "GroupName")]
    pub(crate) group_name: Option<types::group_name_type::GroupNameType>,
    #[serde(rename = "PolicyName")]
    pub(crate) policy_name: Option<types::policy_name_type::PolicyNameType>,
}

impl DeleteGroupPolicyRequest {
    pub(crate) fn group_name(&self) -> Option<&str> {
        self.group_name.as_deref()
    }
    pub(crate) fn policy_name(&self) -> Option<&str> {
        self.policy_name.as_deref()
    }
}

impl validators::NamedValidator for &DeleteGroupPolicyRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_required(self.group_name(), format!("{at}.{}", "GroupName").as_str())?;
        validators::validate_named(self.group_name.as_ref(), format!("{at}.{}", "GroupName").as_str())?;
        validators::validate_required(self.policy_name(), format!("{at}.{}", "PolicyName").as_str())?;
        validators::validate_named(self.policy_name.as_ref(), format!("{at}.{}", "PolicyName").as_str())?;
        Ok(())
    }
}
