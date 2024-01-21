#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) enum EncodingType {
    #[serde(rename = "SSH")]
    Ssh,
    #[serde(rename = "PEM")]
    Pem,
}
