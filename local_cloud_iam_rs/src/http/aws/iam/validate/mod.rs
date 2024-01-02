use crate::http::aws::iam::validate::error::ValidationError;

pub(crate) mod create_policy;
pub(crate) mod error;
pub(crate) mod policy_document;
pub(crate) mod tag;

/// The trait provides base API for source validation. If structure needs to be validate,
/// it must implement this trait.
/// The intention of the validator is to use Fail-Fast approach.
pub(crate) trait IamValidator {
    fn validate(&self) -> Result<(), ValidationError>;
}
