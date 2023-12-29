use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct LocalTag {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,
}
