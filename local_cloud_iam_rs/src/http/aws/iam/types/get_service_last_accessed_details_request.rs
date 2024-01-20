use crate::http::aws::iam::types;
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct GetServiceLastAccessedDetailsRequest {
    #[serde(rename = "MaxItems")]
    pub(crate) max_items: Option<types::max_items_type::MaxItemsType>,
    #[serde(rename = "Marker")]
    pub(crate) marker: Option<types::marker_type::MarkerType>,
    #[serde(rename = "JobId")]
    pub(crate) job_id: Option<types::job_id_type::JobIdType>,
}
impl GetServiceLastAccessedDetailsRequest {
    pub(crate) fn max_items(&self) -> Option<&i32> {
        self.max_items.as_deref()
    }
    pub(crate) fn marker(&self) -> Option<&str> {
        self.marker.as_deref()
    }
    pub(crate) fn job_id(&self) -> Option<&str> {
        self.job_id.as_deref()
    }
}
impl local_cloud_validate::NamedValidator for &GetServiceLastAccessedDetailsRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_named(
            self.max_items.as_ref(),
            format!("{at}.{}", "MaxItems").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.marker.as_ref(),
            format!("{at}.{}", "Marker").as_str(),
        )?;
        local_cloud_validate::validate_required(
            self.job_id(),
            format!("{at}.{}", "JobId").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.job_id.as_ref(),
            format!("{at}.{}", "JobId").as_str(),
        )?;
        Ok(())
    }
}
