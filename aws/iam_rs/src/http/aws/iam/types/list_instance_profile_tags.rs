use crate::http::aws::iam::types;
use crate::http::aws::iam::types::marker_type::MarkerType;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct ListInstanceProfileTagsRequest {
    #[serde(rename = "Marker")]
    pub(crate) marker: Option<MarkerType>,
    #[serde(rename = "MaxItems")]
    pub(crate) max_items: Option<types::max_items_type::MaxItemsType>,
    #[serde(rename = "InstanceProfileName")]
    pub(crate) instance_profile_name: Option<types::instance_profile_name_type::InstanceProfileNameType>,
}

impl ListInstanceProfileTagsRequest {
    pub(crate) fn marker(&self) -> Option<&str> {
        self.marker.as_deref()
    }
    pub(crate) fn marker_type(&self) -> Option<&MarkerType> {
        self.marker.as_ref()
    }
    pub(crate) fn max_items(&self) -> Option<&i32> {
        self.max_items.as_deref()
    }
    pub(crate) fn instance_profile_name(&self) -> Option<&str> {
        self.instance_profile_name.as_deref()
    }
}

impl validators::NamedValidator for &ListInstanceProfileTagsRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_named(self.marker.as_ref(), format!("{at}.{}", "Marker").as_str())?;
        validators::validate_named(self.max_items.as_ref(), format!("{at}.{}", "MaxItems").as_str())?;
        validators::validate_required(
            self.instance_profile_name(),
            format!("{at}.{}", "InstanceProfileName").as_str(),
        )?;
        validators::validate_named(
            self.instance_profile_name.as_ref(),
            format!("{at}.{}", "InstanceProfileName").as_str(),
        )?;
        Ok(())
    }
}
