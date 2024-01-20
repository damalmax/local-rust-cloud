use crate::http::aws::iam::types;
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct GetPolicyRequest {
    #[serde(rename = "PolicyArn")]
    pub(crate) policy_arn: Option<types::arn_type::ArnType>,
}
impl GetPolicyRequest {
    pub(crate) fn policy_arn(&self) -> Option<&str> {
        self.policy_arn.as_deref()
    }
}
impl local_cloud_validate::NamedValidator for &GetPolicyRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
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
