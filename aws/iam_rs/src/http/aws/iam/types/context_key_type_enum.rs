#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) enum ContextKeyTypeEnum {
    #[serde(rename = "stringList")]
    StringList,
    #[serde(rename = "binaryList")]
    BinaryList,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "numeric")]
    Numeric,
    #[serde(rename = "booleanList")]
    BooleanList,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "numericList")]
    NumericList,
    #[serde(rename = "binary")]
    Binary,
    #[serde(rename = "ip")]
    Ip,
    #[serde(rename = "dateList")]
    DateList,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "ipList")]
    IpList,
}
