use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StringShape {
    pub traits: Option<StringTraits>,
}

impl StringShape {
    pub fn documentation(&self) -> Option<String> {
        self.traits.as_ref().and_then(|t| t.documentation.clone())
    }

    pub fn is_required(&self) -> bool {
        if let Some(traits) = &self.traits {
            return traits.sensitive.is_some();
        }
        return false;
    }

    pub fn is_sensitive(&self) -> bool {
        if let Some(traits) = &self.traits {
            return traits.sensitive.is_some();
        }
        return false;
    }

    pub fn length(&self) -> Option<StringLength> {
        if let Some(traits) = &self.traits {
            traits.length
        } else {
            Option::None
        }
    }

    pub fn pattern(&self) -> Option<String> {
        self.traits.as_ref().and_then(|t| t.pattern.clone())
    }

    pub fn is_enum(&self) -> bool {
        if let Some(traits) = &self.traits {
            return traits.enum_value.is_some();
        }
        return false;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StringTraits {
    #[serde(rename = "smithy.api#documentation")]
    pub documentation: Option<String>,
    #[serde(rename = "smithy.api#length")]
    pub length: Option<StringLength>,
    #[serde(rename = "smithy.api#sensitive")]
    pub sensitive: Option<()>,
    #[serde(rename = "smithy.api#pattern")]
    pub pattern: Option<String>,
    #[serde(rename = "smithy.api#enum")]
    pub enum_value: Option<Vec<StringTraitsEnum>>,
    #[serde(rename = "smithy.api#required")]
    pub required: Option<()>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StringTraitsEnum {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct StringLength {
    pub min: Option<i32>,
    pub max: Option<i32>,
}
