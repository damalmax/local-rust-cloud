use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct GetMfaDeviceRequest {
    #[serde(rename = "SerialNumber")]
    pub(crate) serial_number: Option<types::serial_number_type::SerialNumberType>,
    #[serde(rename = "UserName")]
    pub(crate) user_name: Option<types::user_name_type::UserNameType>,
}

impl GetMfaDeviceRequest {
    pub(crate) fn serial_number(&self) -> Option<&str> {
        self.serial_number.as_deref()
    }
    pub(crate) fn user_name(&self) -> Option<&str> {
        self.user_name.as_deref()
    }
}

impl validators::NamedValidator for &GetMfaDeviceRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_required(self.serial_number(), format!("{at}.{}", "SerialNumber").as_str())?;
        validators::validate_named(self.serial_number.as_ref(), format!("{at}.{}", "SerialNumber").as_str())?;
        validators::validate_named(self.user_name.as_ref(), format!("{at}.{}", "UserName").as_str())?;
        Ok(())
    }
}
