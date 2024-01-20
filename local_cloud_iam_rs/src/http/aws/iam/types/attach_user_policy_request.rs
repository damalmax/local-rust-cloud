use crate::http::aws::iam::types;
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct AttachUserPolicyRequest {
    #[serde(rename = "UserName")]
    pub(crate) user_name: Option<types::user_name_type::UserNameType>,
    #[serde(rename = "PolicyArn")]
    pub(crate) policy_arn: Option<types::arn_type::ArnType>,
}
impl AttachUserPolicyRequest {
    pub(crate) fn user_name(&self) -> Option<&str> {
        self.user_name.as_deref()
    }
    pub(crate) fn policy_arn(&self) -> Option<&str> {
        self.policy_arn.as_deref()
    }
}
impl local_cloud_validate::NamedValidator for &AttachUserPolicyRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_required(
            self.user_name(),
            format!("{at}.{}", "UserName").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.user_name.as_ref(),
            format!("{at}.{}", "UserName").as_str(),
        )?;
        local_cloud_validate::validate_required(
            self.policy_arn(),
            format!("{at}.{}", "PolicyArn").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.policy_arn.as_ref(),
            format!("{at}.{}", "PolicyArn").as_str(),
        )?;
        Ok(())
    }
}
