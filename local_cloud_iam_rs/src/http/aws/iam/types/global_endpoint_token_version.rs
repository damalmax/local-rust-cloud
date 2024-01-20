#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) enum GlobalEndpointTokenVersion {
    #[serde(rename = "v1Token")]
    V1Token,
    #[serde(rename = "v2Token")]
    V2Token,
}
