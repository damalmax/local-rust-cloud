use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct EnableMfaDeviceRequest {
    #[serde(rename = "UserName")]
    pub(crate) user_name: Option<types::existing_user_name_type::ExistingUserNameType>,
    #[serde(rename = "SerialNumber")]
    pub(crate) serial_number: Option<types::serial_number_type::SerialNumberType>,
    #[serde(rename = "AuthenticationCode1")]
    pub(crate) authentication_code_1: Option<types::authentication_code_type::AuthenticationCodeType>,
    #[serde(rename = "AuthenticationCode2")]
    pub(crate) authentication_code_2: Option<types::authentication_code_type::AuthenticationCodeType>,
}

impl EnableMfaDeviceRequest {
    pub(crate) fn user_name(&self) -> Option<&str> {
        self.user_name.as_deref()
    }
    pub(crate) fn serial_number(&self) -> Option<&str> {
        self.serial_number.as_deref()
    }
    pub(crate) fn authentication_code_1(&self) -> Option<&str> {
        self.authentication_code_1.as_deref()
    }
    pub(crate) fn authentication_code_2(&self) -> Option<&str> {
        self.authentication_code_2.as_deref()
    }
}

impl local_cloud_validate::NamedValidator for &EnableMfaDeviceRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_required(self.user_name(), format!("{at}.{}", "UserName").as_str())?;
        local_cloud_validate::validate_named(self.user_name.as_ref(), format!("{at}.{}", "UserName").as_str())?;
        local_cloud_validate::validate_required(self.serial_number(), format!("{at}.{}", "SerialNumber").as_str())?;
        local_cloud_validate::validate_named(self.serial_number.as_ref(), format!("{at}.{}", "SerialNumber").as_str())?;
        local_cloud_validate::validate_required(
            self.authentication_code_1(),
            format!("{at}.{}", "AuthenticationCode1").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.authentication_code_1.as_ref(),
            format!("{at}.{}", "AuthenticationCode1").as_str(),
        )?;
        local_cloud_validate::validate_required(
            self.authentication_code_2(),
            format!("{at}.{}", "AuthenticationCode2").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.authentication_code_2.as_ref(),
            format!("{at}.{}", "AuthenticationCode2").as_str(),
        )?;
        Ok(())
    }
}
