use crate::http::aws::iam::types;
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct DeletePolicyVersionRequest {
    #[serde(rename = "PolicyArn")]
    pub(crate) policy_arn: Option<types::arn_type::ArnType>,
    #[serde(rename = "VersionId")]
    pub(crate) version_id: Option<types::policy_version_id_type::PolicyVersionIdType>,
}
impl DeletePolicyVersionRequest {
    pub(crate) fn policy_arn(&self) -> Option<&str> {
        self.policy_arn.as_deref()
    }
    pub(crate) fn version_id(&self) -> Option<&str> {
        self.version_id.as_deref()
    }
}
impl local_cloud_validate::NamedValidator for &DeletePolicyVersionRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_required(self.policy_arn(), format!("{at}.{}", "PolicyArn").as_str())?;
        local_cloud_validate::validate_named(self.policy_arn.as_ref(), format!("{at}.{}", "PolicyArn").as_str())?;
        local_cloud_validate::validate_required(self.version_id(), format!("{at}.{}", "VersionId").as_str())?;
        local_cloud_validate::validate_named(self.version_id.as_ref(), format!("{at}.{}", "VersionId").as_str())?;
        Ok(())
    }
}
