use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct AttachGroupPolicyRequest {
    #[serde(rename = "PolicyArn")]
    pub(crate) policy_arn: Option<types::arn_type::ArnType>,
    #[serde(rename = "GroupName")]
    pub(crate) group_name: Option<types::group_name_type::GroupNameType>,
}

impl AttachGroupPolicyRequest {
    pub(crate) fn policy_arn(&self) -> Option<&str> {
        self.policy_arn.as_deref()
    }
    pub(crate) fn group_name(&self) -> Option<&str> {
        self.group_name.as_deref()
    }
}

impl validators::NamedValidator for &AttachGroupPolicyRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_required(self.policy_arn(), format!("{at}.{}", "PolicyArn").as_str())?;
        validators::validate_named(self.policy_arn.as_ref(), format!("{at}.{}", "PolicyArn").as_str())?;
        validators::validate_required(self.group_name(), format!("{at}.{}", "GroupName").as_str())?;
        validators::validate_named(self.group_name.as_ref(), format!("{at}.{}", "GroupName").as_str())?;
        Ok(())
    }
}
