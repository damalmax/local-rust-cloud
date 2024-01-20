#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) enum StatusType {
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "Inactive")]
    Inactive,
}
