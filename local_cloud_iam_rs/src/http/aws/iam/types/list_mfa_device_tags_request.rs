use local_cloud_validate::{validate_named, validate_required};

use crate::http::aws::iam::types;
use crate::http::aws::iam::types::marker_type::MarkerType;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct ListMfaDeviceTagsRequest {
    #[serde(rename = "MaxItems")]
    pub(crate) max_items: Option<types::max_items_type::MaxItemsType>,
    #[serde(rename = "SerialNumber")]
    pub(crate) serial_number: Option<types::serial_number_type::SerialNumberType>,
    #[serde(rename = "Marker")]
    pub(crate) marker: Option<MarkerType>,
}

impl ListMfaDeviceTagsRequest {
    pub(crate) fn max_items(&self) -> Option<&i32> {
        self.max_items.as_deref()
    }
    pub(crate) fn serial_number(&self) -> Option<&str> {
        self.serial_number.as_deref()
    }
    pub(crate) fn marker(&self) -> Option<&str> {
        self.marker.as_deref()
    }
    pub(crate) fn marker_type(&self) -> Option<&MarkerType> {
        self.marker.as_ref()
    }
}

impl local_cloud_validate::NamedValidator for &ListMfaDeviceTagsRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        validate_named(self.max_items.as_ref(), format!("{at}.{}", "MaxItems").as_str())?;
        validate_required(self.serial_number(), format!("{at}.{}", "SerialNumber").as_str())?;
        validate_named(self.serial_number.as_ref(), format!("{at}.{}", "SerialNumber").as_str())?;
        validate_named(self.marker.as_ref(), format!("{at}.{}", "Marker").as_str())?;
        Ok(())
    }
}
