use std::ops::Deref;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct ServiceName(String);

impl Deref for ServiceName {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl validators::NamedValidator for &ServiceName {
    fn validate(&self, _at: &str) -> Result<(), validators::ValidationError> {
        Ok(())
    }
}
