use serde::Deserialize;

use local_cloud_validate::{validate_required, ValidationError, Validator};

use crate::http::aws::iam::actions::types::arn::ArnType;

#[derive(Debug, Deserialize)]
pub(crate) struct CreatePolicyVersionType {
    #[serde(rename = "PolicyArn")]
    pub(crate) policy_arn: Option<ArnType>,
    #[serde(rename = "PolicyDocument")]
    pub(crate) policy_document: Option<String>,
    #[serde(rename = "IsDefaultVersion")]
    pub(crate) default_version: Option<bool>,
}

impl CreatePolicyVersionType {
    pub(crate) fn policy_arn(&self) -> Option<&str> {
        self.policy_arn.as_deref()
    }

    pub(crate) fn policy_document(&self) -> Option<&str> {
        self.policy_document.as_deref()
    }

    pub(crate) fn default_version(&self) -> Option<bool> {
        self.default_version
    }
}

impl Validator for CreatePolicyVersionType {
    fn validate(&self) -> Result<(), ValidationError> {
        validate_required(self.policy_arn(), "PolicyArn")?;
        validate_required(self.policy_document(), "PolicyDocument")?;
        Ok(())
    }
}
