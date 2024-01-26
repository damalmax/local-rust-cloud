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

impl local_cloud_validate::NamedValidator for &DeleteVirtualMfaDeviceRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_required(self.serial_number(), format!("{at}.{}", "SerialNumber").as_str())?;
        local_cloud_validate::validate_named(self.serial_number.as_ref(), format!("{at}.{}", "SerialNumber").as_str())?;
        Ok(())
    }
}
