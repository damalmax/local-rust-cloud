use serde::Deserialize;

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
}
