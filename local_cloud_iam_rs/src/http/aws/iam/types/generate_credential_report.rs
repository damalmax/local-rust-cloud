use local_cloud_validate::{NamedValidator, ValidationError};

#[derive(Debug, PartialEq, serde::Deserialize)]

pub(crate) struct GenerateCredentialReportRequest(());

impl NamedValidator for GenerateCredentialReportRequest {
    fn validate(&self, _at: &str) -> Result<(), ValidationError> {
        Ok(())
    }
}
