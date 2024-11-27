use validators::{NamedValidator, ValidationError};

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct DeleteAccountPasswordPolicyRequest(());

impl NamedValidator for DeleteAccountPasswordPolicyRequest {
    fn validate(&self, _at: &str) -> Result<(), ValidationError> {
        Ok(())
    }
}
