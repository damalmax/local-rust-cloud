use validators::{validate_array_size_max, validate_array_size_min, validate_named, validate_required};

use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct CreateVirtualMfaDeviceRequest {
    #[serde(rename = "Path")]
    pub(crate) path: Option<types::path_type::PathType>,
    #[serde(rename = "Tags")]
    pub(crate) tags: Option<Vec<types::tag::Tag>>,
    #[serde(rename = "VirtualMFADeviceName")]
    pub(crate) virtual_mfa_device_name: Option<types::virtual_mfa_device_name::VirtualMfaDeviceName>,
}

impl CreateVirtualMfaDeviceRequest {
    pub(crate) fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }
    pub(crate) fn tags(&self) -> Option<&[types::tag::Tag]> {
        self.tags.as_deref()
    }
    pub(crate) fn virtual_mfa_device_name(&self) -> Option<&str> {
        self.virtual_mfa_device_name.as_deref()
    }
}

impl validators::NamedValidator for &CreateVirtualMfaDeviceRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validate_named(self.path.as_ref(), format!("{at}.{}", "Path").as_str())?;
        validate_array_size_min(self.tags(), 0usize, format!("{at}.{}", "Tags").as_str())?;
        validate_array_size_max(self.tags(), 50usize, format!("{at}.{}", "Tags").as_str())?;
        if let Some(tags) = self.tags() {
            for (id, member) in tags.iter().enumerate() {
                validate_named(Some(member), format!("{at}.{}.member.{id}", "Tags").as_str())?;
            }
        }
        validate_required(self.virtual_mfa_device_name(), format!("{at}.{}", "VirtualMFADeviceName").as_str())?;
        validate_named(self.virtual_mfa_device_name.as_ref(), format!("{at}.{}", "VirtualMFADeviceName").as_str())?;
        Ok(())
    }
}
