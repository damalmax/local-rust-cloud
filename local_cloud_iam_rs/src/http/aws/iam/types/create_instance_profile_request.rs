use crate::http::aws::iam::types;
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct CreateInstanceProfileRequest {
    #[serde(rename = "InstanceProfileName")]
    pub(crate) instance_profile_name: Option<types::instance_profile_name_type::InstanceProfileNameType>,
    #[serde(rename = "Tags")]
    pub(crate) tags: Option<Vec<types::tag::Tag>>,
    #[serde(rename = "Path")]
    pub(crate) path: Option<types::path_type::PathType>,
}
impl CreateInstanceProfileRequest {
    pub(crate) fn instance_profile_name(&self) -> Option<&str> {
        self.instance_profile_name.as_deref()
    }
    pub(crate) fn tags(&self) -> Option<&[types::tag::Tag]> {
        self.tags.as_deref()
    }
    pub(crate) fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }
}
impl local_cloud_validate::NamedValidator for &CreateInstanceProfileRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_required(
            self.instance_profile_name(),
            format!("{at}.{}", "InstanceProfileName").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.instance_profile_name.as_ref(),
            format!("{at}.{}", "InstanceProfileName").as_str(),
        )?;
        local_cloud_validate::validate_array_size_min(self.tags(), 0usize, format!("{at}.{}", "Tags").as_str())?;
        local_cloud_validate::validate_array_size_max(self.tags(), 50usize, format!("{at}.{}", "Tags").as_str())?;
        if let Some(tags) = self.tags() {
            for (id, member) in tags.iter().enumerate() {
                local_cloud_validate::validate_named(Some(member), format!("{at}.{}.member.{id}", "Tags").as_str())?;
            }
        }
        local_cloud_validate::validate_named(self.path.as_ref(), format!("{at}.{}", "Path").as_str())?;
        Ok(())
    }
}
