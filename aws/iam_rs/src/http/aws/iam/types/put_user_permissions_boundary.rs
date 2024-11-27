use crate::http::aws::iam::types;
use validators::{validate_named, validate_required, ValidationError};

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct PutUserPermissionsBoundaryRequest {
    #[serde(rename = "PermissionsBoundary")]
    pub(crate) permissions_boundary: Option<types::arn_type::ArnType>,
    #[serde(rename = "UserName")]
    pub(crate) user_name: Option<types::user_name_type::UserNameType>,
}

impl PutUserPermissionsBoundaryRequest {
    pub(crate) fn permissions_boundary(&self) -> Option<&str> {
        self.permissions_boundary.as_deref()
    }
    pub(crate) fn user_name(&self) -> Option<&str> {
        self.user_name.as_deref()
    }
}

impl validators::NamedValidator for &PutUserPermissionsBoundaryRequest {
    fn validate(&self, at: &str) -> Result<(), ValidationError> {
        validate_required(self.permissions_boundary(), format!("{at}.{}", "PermissionsBoundary").as_str())?;
        validate_named(self.permissions_boundary.as_ref(), format!("{at}.{}", "PermissionsBoundary").as_str())?;
        validate_required(self.user_name(), format!("{at}.{}", "UserName").as_str())?;
        validate_named(self.user_name.as_ref(), format!("{at}.{}", "UserName").as_str())?;
        Ok(())
    }
}
