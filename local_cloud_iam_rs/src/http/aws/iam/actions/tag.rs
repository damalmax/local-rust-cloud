use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct LocalTag {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,
}

impl LocalTag {
    pub(crate) fn key(&self) -> Option<&str> {
        self.key.as_deref()
    }

    pub(crate) fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }
}
