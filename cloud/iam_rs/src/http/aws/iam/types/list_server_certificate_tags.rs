use validators::{validate_named, validate_required, ValidationError};

use crate::http::aws::iam::types;
use crate::http::aws::iam::types::marker_type::MarkerType;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct ListServerCertificateTagsRequest {
    #[serde(rename = "ServerCertificateName")]
    pub(crate) server_certificate_name: Option<types::server_certificate_name_type::ServerCertificateNameType>,
    #[serde(rename = "MaxItems")]
    pub(crate) max_items: Option<types::max_items_type::MaxItemsType>,
    #[serde(rename = "Marker")]
    pub(crate) marker: Option<MarkerType>,
}

impl ListServerCertificateTagsRequest {
    pub(crate) fn server_certificate_name(&self) -> Option<&str> {
        self.server_certificate_name.as_deref()
    }
    pub(crate) fn max_items(&self) -> Option<&i32> {
        self.max_items.as_deref()
    }
    pub(crate) fn marker(&self) -> Option<&str> {
        self.marker.as_deref()
    }
    pub(crate) fn marker_type(&self) -> Option<&MarkerType> {
        self.marker.as_ref()
    }
}

impl validators::NamedValidator for &ListServerCertificateTagsRequest {
    fn validate(&self, at: &str) -> Result<(), ValidationError> {
        validate_required(self.server_certificate_name(), format!("{at}.{}", "ServerCertificateName").as_str())?;
        validate_named(self.server_certificate_name.as_ref(), format!("{at}.{}", "ServerCertificateName").as_str())?;
        validate_named(self.max_items.as_ref(), format!("{at}.{}", "MaxItems").as_str())?;
        validate_named(self.marker.as_ref(), format!("{at}.{}", "Marker").as_str())?;
        Ok(())
    }
}
