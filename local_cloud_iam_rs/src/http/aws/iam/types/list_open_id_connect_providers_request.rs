#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct ListOpenIdConnectProvidersRequest {}

impl ListOpenIdConnectProvidersRequest {}

impl local_cloud_validate::NamedValidator for &ListOpenIdConnectProvidersRequest {
    fn validate(&self, _at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        Ok(())
    }
}
