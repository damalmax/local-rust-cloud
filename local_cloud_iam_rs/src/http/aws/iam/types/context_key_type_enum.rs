#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) enum ContextKeyTypeEnum {
    #[serde(rename = "numericList")]
    NumericList,
    #[serde(rename = "dateList")]
    DateList,
    #[serde(rename = "numeric")]
    Numeric,
    #[serde(rename = "stringList")]
    StringList,
    #[serde(rename = "binary")]
    Binary,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "ip")]
    Ip,
    #[serde(rename = "booleanList")]
    BooleanList,
    #[serde(rename = "ipList")]
    IpList,
    #[serde(rename = "binaryList")]
    BinaryList,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "string")]
    String,
}
