use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct DeleteInstanceProfileRequest {
    #[serde(rename = "InstanceProfileName")]
    pub(crate) instance_profile_name: Option<types::instance_profile_name_type::InstanceProfileNameType>,
}

impl DeleteInstanceProfileRequest {
    pub(crate) fn instance_profile_name(&self) -> Option<&str> {
        self.instance_profile_name.as_deref()
    }
}

impl validators::NamedValidator for &DeleteInstanceProfileRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
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
