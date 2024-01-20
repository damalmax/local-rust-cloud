#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) enum AccessAdvisorUsageGranularityType {
    #[serde(rename = "ACTION_LEVEL")]
    ActionLevel,
    #[serde(rename = "SERVICE_LEVEL")]
    ServiceLevel,
}
