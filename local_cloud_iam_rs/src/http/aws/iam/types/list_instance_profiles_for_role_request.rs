use crate::http::aws::iam::types;
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct ListInstanceProfilesForRoleRequest {
    #[serde(rename = "RoleName")]
    pub(crate) role_name: Option<types::role_name_type::RoleNameType>,
    #[serde(rename = "MaxItems")]
    pub(crate) max_items: Option<types::max_items_type::MaxItemsType>,
    #[serde(rename = "Marker")]
    pub(crate) marker: Option<types::marker_type::MarkerType>,
}
impl ListInstanceProfilesForRoleRequest {
    pub(crate) fn role_name(&self) -> Option<&str> {
        self.role_name.as_deref()
    }
    pub(crate) fn max_items(&self) -> Option<&i32> {
        self.max_items.as_deref()
    }
    pub(crate) fn marker(&self) -> Option<&str> {
        self.marker.as_deref()
    }
}
impl local_cloud_validate::NamedValidator for &ListInstanceProfilesForRoleRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_required(
            self.role_name(),
            format!("{at}.{}", "RoleName").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.role_name.as_ref(),
            format!("{at}.{}", "RoleName").as_str(),
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
