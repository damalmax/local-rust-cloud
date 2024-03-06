use local_cloud_validate::validate_named;

use crate::http::aws::iam::types;
use crate::http::aws::iam::types::marker_type::MarkerType;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct ListServerCertificatesRequest {
    #[serde(rename = "MaxItems")]
    pub(crate) max_items: Option<types::max_items_type::MaxItemsType>,
    #[serde(rename = "PathPrefix")]
    pub(crate) path_prefix: Option<types::path_prefix_type::PathPrefixType>,
    #[serde(rename = "Marker")]
    pub(crate) marker: Option<MarkerType>,
}

impl ListServerCertificatesRequest {
    pub(crate) fn max_items(&self) -> Option<&i32> {
        self.max_items.as_deref()
    }
    pub(crate) fn path_prefix(&self) -> Option<&str> {
        self.path_prefix.as_deref()
    }
    pub(crate) fn marker(&self) -> Option<&str> {
        self.marker.as_deref()
    }
    pub(crate) fn marker_type(&self) -> Option<&MarkerType> {
        self.marker.as_ref()
    }
}

impl local_cloud_validate::NamedValidator for &ListServerCertificatesRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        validate_named(self.max_items.as_ref(), format!("{at}.{}", "MaxItems").as_str())?;
        validate_named(self.path_prefix.as_ref(), format!("{at}.{}", "PathPrefix").as_str())?;
        validate_named(self.marker.as_ref(), format!("{at}.{}", "Marker").as_str())?;
        Ok(())
    }
}
