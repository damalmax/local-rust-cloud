use std::ops::Deref;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct ContextKeyValueType(String);

impl Deref for ContextKeyValueType {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl validators::NamedValidator for &ContextKeyValueType {
    fn validate(&self, _at: &str) -> Result<(), validators::ValidationError> {
        Ok(())
    }
}
