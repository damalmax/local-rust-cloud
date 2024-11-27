use validators::{NamedValidator, ValidationError};

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct GetAccountPasswordPolicyRequest(());

impl NamedValidator for GetAccountPasswordPolicyRequest {
    fn validate(&self, _at: &str) -> Result<(), ValidationError> {
        Ok(())
    }
}
