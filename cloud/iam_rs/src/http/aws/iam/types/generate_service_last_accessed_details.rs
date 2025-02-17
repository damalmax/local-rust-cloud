use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct GenerateServiceLastAccessedDetailsRequest {
    #[serde(rename = "Arn")]
    pub(crate) arn: Option<types::arn_type::ArnType>,
    #[serde(rename = "Granularity")]
    pub(crate) granularity: Option<types::access_advisor_usage_granularity_type::AccessAdvisorUsageGranularityType>,
}

impl GenerateServiceLastAccessedDetailsRequest {
    pub(crate) fn arn(&self) -> Option<&str> {
        self.arn.as_deref()
    }
    pub(crate) fn granularity(
        &self,
    ) -> Option<&types::access_advisor_usage_granularity_type::AccessAdvisorUsageGranularityType> {
        self.granularity.as_ref()
    }
}

impl validators::NamedValidator for &GenerateServiceLastAccessedDetailsRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_required(self.arn(), format!("{at}.{}", "Arn").as_str())?;
        validators::validate_named(self.arn.as_ref(), format!("{at}.{}", "Arn").as_str())?;
        Ok(())
    }
}
