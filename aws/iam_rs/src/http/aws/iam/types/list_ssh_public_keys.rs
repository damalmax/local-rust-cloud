use validators::validate_named;

use crate::http::aws::iam::types;
use crate::http::aws::iam::types::marker_type::MarkerType;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct ListSshPublicKeysRequest {
    #[serde(rename = "UserName")]
    pub(crate) user_name: Option<types::user_name_type::UserNameType>,
    #[serde(rename = "MaxItems")]
    pub(crate) max_items: Option<types::max_items_type::MaxItemsType>,
    #[serde(rename = "Marker")]
    pub(crate) marker: Option<MarkerType>,
}

impl ListSshPublicKeysRequest {
    pub(crate) fn user_name(&self) -> Option<&str> {
        self.user_name.as_deref()
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

impl validators::NamedValidator for &ListSshPublicKeysRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validate_named(self.user_name.as_ref(), format!("{at}.{}", "UserName").as_str())?;
        validate_named(self.max_items.as_ref(), format!("{at}.{}", "MaxItems").as_str())?;
        validate_named(self.marker.as_ref(), format!("{at}.{}", "Marker").as_str())?;
        Ok(())
    }
}
