use serde::Deserialize;

mod action;
mod output;

#[derive(Debug, Deserialize)]
pub(crate) struct LocalCreatePolicyVersion {
    #[serde(rename = "PolicyArn")]
    pub(crate) policy_arn: Option<String>,
    #[serde(rename = "PolicyDocument")]
    pub(crate) policy_document: Option<String>,
    #[serde(rename = "IsDefaultVersion")]
    pub(crate) default_version: Option<bool>,
}

impl LocalCreatePolicyVersion {
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
