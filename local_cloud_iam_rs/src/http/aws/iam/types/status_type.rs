#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) enum StatusType {
    #[serde(rename = "Inactive")]
    Inactive,
    #[serde(rename = "Active")]
    Active,
}
