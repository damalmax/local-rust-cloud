#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct ListSamlProvidersRequest {}

impl validators::NamedValidator for &ListSamlProvidersRequest {
    fn validate(&self, _at: &str) -> Result<(), validators::ValidationError> {
        Ok(())
    }
}
