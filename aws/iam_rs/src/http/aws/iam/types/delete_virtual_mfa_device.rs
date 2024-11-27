use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct DeleteVirtualMfaDeviceRequest {
    #[serde(rename = "SerialNumber")]
    pub(crate) serial_number: Option<types::serial_number_type::SerialNumberType>,
}

impl DeleteVirtualMfaDeviceRequest {
    pub(crate) fn serial_number(&self) -> Option<&str> {
        self.serial_number.as_deref()
    }
}

impl validators::NamedValidator for &DeleteVirtualMfaDeviceRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_required(self.serial_number(), format!("{at}.{}", "SerialNumber").as_str())?;
        validators::validate_named(self.serial_number.as_ref(), format!("{at}.{}", "SerialNumber").as_str())?;
        Ok(())
    }
}
