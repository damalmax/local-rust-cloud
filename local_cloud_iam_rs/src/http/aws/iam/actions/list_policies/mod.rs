use serde::Deserialize;

use crate::http::aws::iam::actions::policy_scope::LocalPolicyScopeType;
use crate::http::aws::iam::actions::policy_usage::LocalPolicyUsageType;
use crate::http::aws::iam::actions::types::marker::MarkerType;

pub(crate) mod action;
pub(crate) mod output;

#[derive(Debug, Deserialize)]
pub(crate) struct LocalListPolicies {
    #[serde(rename = "Scope")]
    pub scope: Option<LocalPolicyScopeType>,
    #[serde(rename = "OnlyAttached")]
    pub only_attached: Option<bool>,
    #[serde(rename = "PathPrefix")]
    pub path_prefix: Option<String>,
    #[serde(rename = "PolicyUsageFilter")]
    pub policy_usage_filter: Option<LocalPolicyUsageType>,
    #[serde(rename = "Marker")]
    pub marker: Option<MarkerType>,
    #[serde(rename = "MaxItems")]
    pub max_items: Option<i32>,
}

impl LocalListPolicies {
    pub(crate) fn marker(&self) -> Option<&str> {
        self.marker.as_deref()
    }
}
