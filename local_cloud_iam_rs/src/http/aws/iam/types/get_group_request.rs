use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct GetGroupRequest {
    #[serde(rename = "MaxItems")]
    pub(crate) max_items: Option<types::max_items_type::MaxItemsType>,
    #[serde(rename = "Marker")]
    pub(crate) marker: Option<types::marker_type::MarkerType>,
    #[serde(rename = "GroupName")]
    pub(crate) group_name: Option<types::group_name_type::GroupNameType>,
}

impl GetGroupRequest {
    pub(crate) fn max_items(&self) -> Option<&i32> {
        self.max_items.as_deref()
    }
    pub(crate) fn marker(&self) -> Option<&str> {
        self.marker.as_deref()
    }
    pub(crate) fn marker_type(&self) -> Option<&types::marker_type::MarkerType> {
        self.marker.as_ref()
    }
    pub(crate) fn group_name(&self) -> Option<&str> {
        self.group_name.as_deref()
    }
}

impl local_cloud_validate::NamedValidator for &GetGroupRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_named(self.max_items.as_ref(), format!("{at}.{}", "MaxItems").as_str())?;
        local_cloud_validate::validate_named(self.marker.as_ref(), format!("{at}.{}", "Marker").as_str())?;
        local_cloud_validate::validate_required(self.group_name(), format!("{at}.{}", "GroupName").as_str())?;
        local_cloud_validate::validate_named(self.group_name.as_ref(), format!("{at}.{}", "GroupName").as_str())?;
        Ok(())
    }
}
