use serde::Deserialize;

use local_cloud_actix::local::web::RequestId;

use crate::http::aws::sts::actions::types::tag::LocalTag;

pub(crate) mod action;
pub(crate) mod output;
pub(crate) mod validate;

#[derive(Debug, Deserialize)]
pub(crate) struct LocalPolicyDescriptorType {}

#[derive(Debug, Deserialize)]
pub(crate) struct LocalProvidedContext {}

#[derive(Debug, Deserialize)]
pub struct LocalAssumeRole {
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,
    #[serde(rename = "RoleSessionName")]
    pub role_session_name: Option<String>,
    #[serde(rename = "PolicyArns")]
    pub policy_arns: Option<Vec<LocalPolicyDescriptorType>>,
    #[serde(rename = "Policy")]
    pub policy: Option<String>,
    #[serde(rename = "DurationSeconds")]
    pub duration_seconds: Option<i32>,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<LocalTag>>,
    #[serde(rename = "TransitiveTagKeys")]
    pub transitive_tag_keys: Option<Vec<String>>,
    #[serde(rename = "ExternalId")]
    pub external_id: Option<String>,
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,
    #[serde(rename = "TokenCode")]
    pub token_code: Option<String>,
    #[serde(rename = "SourceIdentity")]
    pub source_identity: Option<String>,
    #[serde(rename = "ProvidedContexts")]
    pub provided_contexts: Option<Vec<LocalProvidedContext>>,
    #[serde(default = "RequestId::default")]
    pub(crate) sts_request_id: RequestId,
}

impl LocalAssumeRole {
    pub fn role_arn(&self) -> Option<&str> {
        self.role_arn.as_deref()
    }

    pub fn role_session_name(&self) -> Option<&str> {
        self.role_session_name.as_deref()
    }

    pub fn policy_arns(&self) -> Option<&[LocalPolicyDescriptorType]> {
        self.policy_arns.as_deref()
    }

    pub fn duration_seconds(&self) -> Option<i32> {
        self.duration_seconds
    }

    pub fn tags(&self) -> Option<&[LocalTag]> {
        self.tags.as_deref()
    }

    pub fn transitive_tag_keys(&self) -> Option<&[String]> {
        self.transitive_tag_keys.as_deref()
    }

    pub fn external_id(&self) -> Option<&str> {
        self.external_id.as_deref()
    }

    pub fn serial_number(&self) -> Option<&str> {
        self.serial_number.as_deref()
    }

    pub fn token_code(&self) -> Option<&str> {
        self.token_code.as_deref()
    }

    pub fn source_identity(&self) -> Option<&str> {
        self.source_identity.as_deref()
    }

    pub fn provided_contexts(&self) -> Option<&[LocalProvidedContext]> {
        self.provided_contexts.as_deref()
    }

    pub fn sts_request_id(&self) -> &str {
        self.sts_request_id.0.as_str()
    }
}
