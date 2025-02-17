use validators::{validate_named, validate_required, ValidationError};

use crate::http::aws::iam::types;
use crate::http::aws::iam::types::marker_type::MarkerType;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct ListAttachedUserPoliciesRequest {
    #[serde(rename = "Marker")]
    pub(crate) marker: Option<types::marker_type::MarkerType>,
    #[serde(rename = "UserName")]
    pub(crate) user_name: Option<types::user_name_type::UserNameType>,
    #[serde(rename = "MaxItems")]
    pub(crate) max_items: Option<types::max_items_type::MaxItemsType>,
    #[serde(rename = "PathPrefix")]
    pub(crate) path_prefix: Option<types::policy_path_type::PolicyPathType>,
}

impl ListAttachedUserPoliciesRequest {
    pub(crate) fn marker(&self) -> Option<&str> {
        self.marker.as_deref()
    }
    pub(crate) fn marker_type(&self) -> Option<&MarkerType> {
        self.marker.as_ref()
    }
    pub(crate) fn user_name(&self) -> Option<&str> {
        self.user_name.as_deref()
    }
    pub(crate) fn max_items(&self) -> Option<&i32> {
        self.max_items.as_deref()
    }
    pub(crate) fn path_prefix(&self) -> Option<&str> {
        self.path_prefix.as_deref()
    }
}

impl validators::NamedValidator for &ListAttachedUserPoliciesRequest {
    fn validate(&self, at: &str) -> Result<(), ValidationError> {
        validate_named(self.marker.as_ref(), format!("{at}.{}", "Marker").as_str())?;
        validate_required(self.user_name(), format!("{at}.{}", "UserName").as_str())?;
        validate_named(self.user_name.as_ref(), format!("{at}.{}", "UserName").as_str())?;
        validate_named(self.max_items.as_ref(), format!("{at}.{}", "MaxItems").as_str())?;
        validate_named(self.path_prefix.as_ref(), format!("{at}.{}", "PathPrefix").as_str())?;
        Ok(())
    }
}
