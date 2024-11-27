use validators::{NamedValidator, ValidationError};

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct GetCredentialReportRequest(());

impl NamedValidator for GetCredentialReportRequest {
    fn validate(&self, _at: &str) -> Result<(), ValidationError> {
        Ok(())
    }
}
