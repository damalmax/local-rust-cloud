use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct ListPoliciesRequest {
    #[serde(rename = "MaxItems")]
    pub(crate) max_items: Option<types::max_items_type::MaxItemsType>,
    #[serde(rename = "PathPrefix")]
    pub(crate) path_prefix: Option<types::policy_path_type::PolicyPathType>,
    #[serde(rename = "OnlyAttached")]
    pub(crate) only_attached: Option<local_cloud_common::types::Bool>,
    #[serde(rename = "Scope")]
    pub(crate) scope: Option<types::policy_scope_type::PolicyScopeType>,
    #[serde(rename = "PolicyUsageFilter")]
    pub(crate) policy_usage_filter: Option<types::policy_usage_type::PolicyUsageType>,
    #[serde(rename = "Marker")]
    pub(crate) marker: Option<types::marker_type::MarkerType>,
}
impl ListPoliciesRequest {
    pub(crate) fn max_items(&self) -> Option<&i32> {
        self.max_items.as_deref()
    }
    pub(crate) fn path_prefix(&self) -> Option<&str> {
        self.path_prefix.as_deref()
    }
    pub(crate) fn only_attached(&self) -> Option<bool> {
        self.only_attached.as_ref().map(|v| v.as_bool())
    }
    pub(crate) fn scope(&self) -> Option<&types::policy_scope_type::PolicyScopeType> {
        self.scope.as_ref()
    }
    pub(crate) fn policy_usage_filter(&self) -> Option<&types::policy_usage_type::PolicyUsageType> {
        self.policy_usage_filter.as_ref()
    }
    pub(crate) fn marker(&self) -> Option<&str> {
        self.marker.as_deref()
    }
}
impl local_cloud_validate::NamedValidator for &ListPoliciesRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_named(self.max_items.as_ref(), format!("{at}.{}", "MaxItems").as_str())?;
        local_cloud_validate::validate_named(self.path_prefix.as_ref(), format!("{at}.{}", "PathPrefix").as_str())?;
        local_cloud_validate::validate_named(self.marker.as_ref(), format!("{at}.{}", "Marker").as_str())?;
        Ok(())
    }
}
