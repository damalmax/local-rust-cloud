use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct PutRolePermissionsBoundaryRequest {
    #[serde(rename = "RoleName")]
    pub(crate) role_name: Option<types::role_name_type::RoleNameType>,
    #[serde(rename = "PermissionsBoundary")]
    pub(crate) permissions_boundary: Option<types::arn_type::ArnType>,
}

impl PutRolePermissionsBoundaryRequest {
    pub(crate) fn role_name(&self) -> Option<&str> {
        self.role_name.as_deref()
    }
    pub(crate) fn permissions_boundary(&self) -> Option<&str> {
        self.permissions_boundary.as_deref()
    }
}

impl local_cloud_validate::NamedValidator for &PutRolePermissionsBoundaryRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_required(self.role_name(), format!("{at}.{}", "RoleName").as_str())?;
        local_cloud_validate::validate_named(self.role_name.as_ref(), format!("{at}.{}", "RoleName").as_str())?;
        local_cloud_validate::validate_required(
            self.permissions_boundary(),
            format!("{at}.{}", "PermissionsBoundary").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.permissions_boundary.as_ref(),
            format!("{at}.{}", "PermissionsBoundary").as_str(),
        )?;
        Ok(())
    }
}
