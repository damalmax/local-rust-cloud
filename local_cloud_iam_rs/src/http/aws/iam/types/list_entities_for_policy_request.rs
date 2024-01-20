use crate::http::aws::iam::types;
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct ListEntitiesForPolicyRequest {
    #[serde(rename = "PolicyArn")]
    pub(crate) policy_arn: Option<types::arn_type::ArnType>,
    #[serde(rename = "PathPrefix")]
    pub(crate) path_prefix: Option<types::path_type::PathType>,
    #[serde(rename = "EntityFilter")]
    pub(crate) entity_filter: Option<types::entity_type::EntityType>,
    #[serde(rename = "PolicyUsageFilter")]
    pub(crate) policy_usage_filter: Option<types::policy_usage_type::PolicyUsageType>,
    #[serde(rename = "MaxItems")]
    pub(crate) max_items: Option<types::max_items_type::MaxItemsType>,
    #[serde(rename = "Marker")]
    pub(crate) marker: Option<types::marker_type::MarkerType>,
}
impl ListEntitiesForPolicyRequest {
    pub(crate) fn policy_arn(&self) -> Option<&str> {
        self.policy_arn.as_deref()
    }
    pub(crate) fn path_prefix(&self) -> Option<&str> {
        self.path_prefix.as_deref()
    }
    pub(crate) fn entity_filter(&self) -> Option<&types::entity_type::EntityType> {
        self.entity_filter.as_ref()
    }
    pub(crate) fn policy_usage_filter(
        &self,
    ) -> Option<&types::policy_usage_type::PolicyUsageType> {
        self.policy_usage_filter.as_ref()
    }
    pub(crate) fn max_items(&self) -> Option<&i32> {
        self.max_items.as_deref()
    }
    pub(crate) fn marker(&self) -> Option<&str> {
        self.marker.as_deref()
    }
}
impl local_cloud_validate::NamedValidator for &ListEntitiesForPolicyRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_required(
            self.policy_arn(),
            format!("{at}.{}", "PolicyArn").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.policy_arn.as_ref(),
            format!("{at}.{}", "PolicyArn").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.path_prefix.as_ref(),
            format!("{at}.{}", "PathPrefix").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.max_items.as_ref(),
            format!("{at}.{}", "MaxItems").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.marker.as_ref(),
            format!("{at}.{}", "Marker").as_str(),
        )?;
        Ok(())
    }
}
