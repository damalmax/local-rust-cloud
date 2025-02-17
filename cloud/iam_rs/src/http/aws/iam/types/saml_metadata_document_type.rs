use std::ops::Deref;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct SamlMetadataDocumentType(String);

impl Deref for SamlMetadataDocumentType {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl validators::NamedValidator for &SamlMetadataDocumentType {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_str_length_min(Some(self), 1000usize, at)?;
        validators::validate_str_length_max(Some(self), 10000000usize, at)?;
        Ok(())
    }
}
