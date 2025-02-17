use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct DeleteGroupRequest {
    #[serde(rename = "GroupName")]
    pub(crate) group_name: Option<types::group_name_type::GroupNameType>,
}

impl DeleteGroupRequest {
    pub(crate) fn group_name(&self) -> Option<&str> {
        self.group_name.as_deref()
    }
}

impl validators::NamedValidator for &DeleteGroupRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_required(self.group_name(), format!("{at}.{}", "GroupName").as_str())?;
        validators::validate_named(self.group_name.as_ref(), format!("{at}.{}", "GroupName").as_str())?;
        Ok(())
    }
}
