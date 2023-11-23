use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StringShape {
    pub traits: Option<StringTraits>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StringTraits {
    #[serde(rename = "smithy.api#length")]
    pub length: Option<StringTraitsLength>,
    #[serde(rename = "smithy.api#sensitive")]
    pub sensitive: Option<()>,
    #[serde(rename = "smithy.api#pattern")]
    pub pattern: Option<String>,
    #[serde(rename = "smithy.api#enum")]
    pub enum_value: Option<Vec<StringTraitsEnum>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StringTraitsEnum {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StringTraitsLength {
    pub min: Option<i32>,
    pub max: Option<i32>,
}