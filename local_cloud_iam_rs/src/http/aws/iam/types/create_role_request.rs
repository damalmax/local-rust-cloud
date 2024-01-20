use crate::http::aws::iam::types;
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct CreateRoleRequest {
    #[serde(rename = "Tags")]
    pub(crate) tags: Option<Vec<types::tag::Tag>>,
    #[serde(rename = "PermissionsBoundary")]
    pub(crate) permissions_boundary: Option<types::arn_type::ArnType>,
    #[serde(rename = "MaxSessionDuration")]
    pub(crate) max_session_duration: Option<
        types::role_max_session_duration_type::RoleMaxSessionDurationType,
    >,
    #[serde(rename = "RoleName")]
    pub(crate) role_name: Option<types::role_name_type::RoleNameType>,
    #[serde(rename = "AssumeRolePolicyDocument")]
    pub(crate) assume_role_policy_document: Option<
        types::policy_document_type::PolicyDocumentType,
    >,
    #[serde(rename = "Path")]
    pub(crate) path: Option<types::path_type::PathType>,
    #[serde(rename = "Description")]
    pub(crate) description: Option<types::role_description_type::RoleDescriptionType>,
}
impl CreateRoleRequest {
    pub(crate) fn tags(&self) -> Option<&[types::tag::Tag]> {
        self.tags.as_deref()
    }
    pub(crate) fn permissions_boundary(&self) -> Option<&str> {
        self.permissions_boundary.as_deref()
    }
    pub(crate) fn max_session_duration(&self) -> Option<&i32> {
        self.max_session_duration.as_deref()
    }
    pub(crate) fn role_name(&self) -> Option<&str> {
        self.role_name.as_deref()
    }
    pub(crate) fn assume_role_policy_document(&self) -> Option<&str> {
        self.assume_role_policy_document.as_deref()
    }
    pub(crate) fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }
    pub(crate) fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}
impl local_cloud_validate::NamedValidator for &CreateRoleRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_array_size_min(
            self.tags(),
            0usize,
            format!("{at}.{}", "Tags").as_str(),
        )?;
        local_cloud_validate::validate_array_size_max(
            self.tags(),
            50usize,
            format!("{at}.{}", "Tags").as_str(),
        )?;
        if let Some(tags) = self.tags() {
            for (id, member) in tags.iter().enumerate() {
                local_cloud_validate::validate_named(
                    Some(member),
                    format!("{at}.{}.member.{id}", "Tags").as_str(),
                )?;
            }
        }
        local_cloud_validate::validate_named(
            self.permissions_boundary.as_ref(),
            format!("{at}.{}", "PermissionsBoundary").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.max_session_duration.as_ref(),
            format!("{at}.{}", "MaxSessionDuration").as_str(),
        )?;
        local_cloud_validate::validate_required(
            self.role_name(),
            format!("{at}.{}", "RoleName").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.role_name.as_ref(),
            format!("{at}.{}", "RoleName").as_str(),
        )?;
        local_cloud_validate::validate_required(
            self.assume_role_policy_document(),
            format!("{at}.{}", "AssumeRolePolicyDocument").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.assume_role_policy_document.as_ref(),
            format!("{at}.{}", "AssumeRolePolicyDocument").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.path.as_ref(),
            format!("{at}.{}", "Path").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.description.as_ref(),
            format!("{at}.{}", "Description").as_str(),
        )?;
        Ok(())
    }
}
