use validators::{validate_named, validate_required};
use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct DeactivateMfaDeviceRequest {
    #[serde(rename = "UserName")]
    pub(crate) user_name: Option<types::existing_user_name_type::ExistingUserNameType>,
    #[serde(rename = "SerialNumber")]
    pub(crate) serial_number: Option<types::serial_number_type::SerialNumberType>,
}

impl DeactivateMfaDeviceRequest {
    pub(crate) fn user_name(&self) -> Option<&str> {
        self.user_name.as_deref()
    }
    pub(crate) fn serial_number(&self) -> Option<&str> {
        self.serial_number.as_deref()
    }
}

impl validators::NamedValidator for &DeactivateMfaDeviceRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validate_required(self.user_name(), format!("{at}.{}", "UserName").as_str())?;
        validate_named(self.user_name.as_ref(), format!("{at}.{}", "UserName").as_str())?;
        validate_required(self.serial_number(), format!("{at}.{}", "SerialNumber").as_str())?;
        validate_named(self.serial_number.as_ref(), format!("{at}.{}", "SerialNumber").as_str())?;
        Ok(())
    }
}
