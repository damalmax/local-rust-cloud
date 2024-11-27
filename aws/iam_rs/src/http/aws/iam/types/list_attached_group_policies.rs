use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct ListAttachedGroupPoliciesRequest {
    #[serde(rename = "PathPrefix")]
    pub(crate) path_prefix: Option<types::policy_path_type::PolicyPathType>,
    #[serde(rename = "GroupName")]
    pub(crate) group_name: Option<types::group_name_type::GroupNameType>,
    #[serde(rename = "Marker")]
    pub(crate) marker: Option<types::marker_type::MarkerType>,
    #[serde(rename = "MaxItems")]
    pub(crate) max_items: Option<types::max_items_type::MaxItemsType>,
}

impl ListAttachedGroupPoliciesRequest {
    pub(crate) fn path_prefix(&self) -> Option<&str> {
        self.path_prefix.as_deref()
    }
    pub(crate) fn group_name(&self) -> Option<&str> {
        self.group_name.as_deref()
    }
    pub(crate) fn marker(&self) -> Option<&str> {
        self.marker.as_deref()
    }
    pub(crate) fn max_items(&self) -> Option<&i32> {
        self.max_items.as_deref()
    }
}

impl validators::NamedValidator for &ListAttachedGroupPoliciesRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_named(self.path_prefix.as_ref(), format!("{at}.{}", "PathPrefix").as_str())?;
        validators::validate_required(self.group_name(), format!("{at}.{}", "GroupName").as_str())?;
        validators::validate_named(self.group_name.as_ref(), format!("{at}.{}", "GroupName").as_str())?;
        validators::validate_named(self.marker.as_ref(), format!("{at}.{}", "Marker").as_str())?;
        validators::validate_named(self.max_items.as_ref(), format!("{at}.{}", "MaxItems").as_str())?;
        Ok(())
    }
}
