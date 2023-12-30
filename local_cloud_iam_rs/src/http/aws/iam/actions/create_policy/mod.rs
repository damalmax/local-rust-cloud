use serde::Deserialize;

use local_cloud_actix::local::web::RequestId;

use crate::http::aws::iam::actions::tag::LocalTag;

pub(crate) mod action;
mod output;
pub(crate) mod validate;

#[derive(Debug, Deserialize)]
pub(crate) struct LocalCreatePolicy {
    #[serde(rename = "PolicyName")]
    pub(crate) policy_name: Option<String>,
    #[serde(rename = "Path")]
    pub(crate) path: Option<String>,
    #[serde(rename = "PolicyDocument")]
    pub(crate) policy_document: Option<String>,
    #[serde(rename = "Description")]
    pub(crate) description: Option<String>,
    #[serde(rename = "Tags")]
    pub(crate) tags: Option<Vec<LocalTag>>,
    #[serde(default = "RequestId::default")]
    pub(crate) aws_request_id: RequestId,
}

impl LocalCreatePolicy {
    pub fn policy_name(&self) -> Option<&str> {
        self.policy_name.as_deref()
    }

    pub fn policy_document(&self) -> Option<&str> {
        self.policy_document.as_deref()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    pub fn tags(&self) -> Option<&[LocalTag]> {
        self.tags.as_deref()
    }

    pub fn aws_request_id(&self) -> &str {
        self.aws_request_id.0.as_str()
    }
}
