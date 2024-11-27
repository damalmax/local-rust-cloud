use crate::http::aws::iam::types;
use validators::{validate_named, validate_required, NamedValidator};

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

impl NamedValidator for &DeletePolicyVersionRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validate_required(self.policy_arn(), format!("{at}.{}", "PolicyArn").as_str())?;
        validate_named(self.policy_arn.as_ref(), format!("{at}.{}", "PolicyArn").as_str())?;
        validate_required(self.version_id(), format!("{at}.{}", "VersionId").as_str())?;
        validate_named(self.version_id.as_ref(), format!("{at}.{}", "VersionId").as_str())?;
        Ok(())
    }
}
