#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) enum PolicyScopeType {
    #[serde(rename = "Local")]
    Local,
    #[serde(rename = "All")]
    All,
    #[serde(rename = "AWS")]
    Aws,
}
