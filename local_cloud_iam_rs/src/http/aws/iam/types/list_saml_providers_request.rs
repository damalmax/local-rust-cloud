#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct ListSamlProvidersRequest {}

impl ListSamlProvidersRequest {}

impl local_cloud_validate::NamedValidator for &ListSamlProvidersRequest {
    fn validate(&self, _at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        Ok(())
    }
}
