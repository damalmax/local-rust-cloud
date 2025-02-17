use validators::validate_named;

use crate::http::aws::iam::types;
use crate::http::aws::iam::types::marker_type::MarkerType;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct ListVirtualMfaDevicesRequest {
    #[serde(rename = "Marker")]
    pub(crate) marker: Option<MarkerType>,
    #[serde(rename = "AssignmentStatus")]
    pub(crate) assignment_status: Option<types::assignment_status_type::AssignmentStatusType>,
    #[serde(rename = "MaxItems")]
    pub(crate) max_items: Option<types::max_items_type::MaxItemsType>,
}

impl ListVirtualMfaDevicesRequest {
    pub(crate) fn marker(&self) -> Option<&str> {
        self.marker.as_deref()
    }
    pub(crate) fn marker_type(&self) -> Option<&MarkerType> {
        self.marker.as_ref()
    }
    pub(crate) fn assignment_status(&self) -> Option<&types::assignment_status_type::AssignmentStatusType> {
        self.assignment_status.as_ref()
    }
    pub(crate) fn max_items(&self) -> Option<&i32> {
        self.max_items.as_deref()
    }
}

impl validators::NamedValidator for &ListVirtualMfaDevicesRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validate_named(self.marker.as_ref(), format!("{at}.{}", "Marker").as_str())?;
        validate_named(self.max_items.as_ref(), format!("{at}.{}", "MaxItems").as_str())?;
        Ok(())
    }
}
