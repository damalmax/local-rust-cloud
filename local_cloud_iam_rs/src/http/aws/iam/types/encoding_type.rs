#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) enum EncodingType {
    #[serde(rename = "PEM")]
    Pem,
    #[serde(rename = "SSH")]
    Ssh,
}
