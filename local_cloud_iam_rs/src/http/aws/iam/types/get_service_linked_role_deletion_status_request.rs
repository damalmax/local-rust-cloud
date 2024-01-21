use crate::http::aws::iam::types;
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct GetServiceLinkedRoleDeletionStatusRequest {
    #[serde(rename = "DeletionTaskId")]
    pub(crate) deletion_task_id: Option<types::deletion_task_id_type::DeletionTaskIdType>,
}
impl GetServiceLinkedRoleDeletionStatusRequest {
    pub(crate) fn deletion_task_id(&self) -> Option<&str> {
        self.deletion_task_id.as_deref()
    }
}
impl local_cloud_validate::NamedValidator for &GetServiceLinkedRoleDeletionStatusRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_required(
            self.deletion_task_id(),
            format!("{at}.{}", "DeletionTaskId").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.deletion_task_id.as_ref(),
            format!("{at}.{}", "DeletionTaskId").as_str(),
        )?;
        Ok(())
    }
}
