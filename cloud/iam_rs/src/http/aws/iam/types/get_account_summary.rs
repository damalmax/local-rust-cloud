use validators::{NamedValidator, ValidationError};

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct GetAccountSummaryRequest(());

impl NamedValidator for GetAccountSummaryRequest {
    fn validate(&self, _at: &str) -> Result<(), ValidationError> {
        Ok(())
    }
}
