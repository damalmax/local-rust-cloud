use serde::Deserialize;

use local_cloud_actix::local::web::RequestId;

use crate::http::aws::iam::actions::tag::LocalTag;

pub(crate) mod action;
mod output;
pub(crate) mod validate;

#[derive(Deserialize, Debug)]
pub(crate) struct LocalCreateUser {
    #[serde(rename = "Path")]
    pub(crate) path: Option<String>,
    #[serde(rename = "UserName")]
    pub(crate) user_name: Option<String>,
    #[serde(rename = "PermissionsBoundary")]
    pub(crate) permissions_boundary: Option<String>,
    #[serde(rename = "Tags")]
    pub(crate) tags: Option<Vec<LocalTag>>,
    #[serde(default = "RequestId::default")]
    pub(crate) aws_request_id: RequestId,
}

impl LocalCreateUser {
    pub(crate) fn aws_request_id(&self) -> &str {
        self.aws_request_id.0.as_str()
    }
}
