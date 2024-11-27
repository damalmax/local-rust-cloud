#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct ListOpenIdConnectProvidersRequest {}

impl ListOpenIdConnectProvidersRequest {}

impl validators::NamedValidator for &ListOpenIdConnectProvidersRequest {
    fn validate(&self, _at: &str) -> Result<(), validators::ValidationError> {
        Ok(())
    }
}
