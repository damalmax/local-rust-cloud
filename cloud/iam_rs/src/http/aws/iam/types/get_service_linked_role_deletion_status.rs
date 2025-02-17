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

impl validators::NamedValidator for &GetServiceLinkedRoleDeletionStatusRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_required(
            self.deletion_task_id(),
            format!("{at}.{}", "DeletionTaskId").as_str(),
        )?;
        validators::validate_named(
            self.deletion_task_id.as_ref(),
            format!("{at}.{}", "DeletionTaskId").as_str(),
        )?;
        Ok(())
    }
}
