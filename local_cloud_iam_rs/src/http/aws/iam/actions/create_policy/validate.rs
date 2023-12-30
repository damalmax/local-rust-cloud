use crate::http::aws::iam::actions::create_policy::LocalCreatePolicy;
use crate::http::aws::iam::actions::error::{IamError, IamErrorKind};
use crate::http::aws::iam::actions::validate::{IamValidator, ValidationError};
use crate::http::aws::iam::constants;

const EMPTY_STR: &str = "";

impl IamValidator for LocalCreatePolicy {
    fn validate(&self) -> Result<(), IamError> {
        self.validate_policy_name()?;
        self.validate_policy_document()?;
        self.validate_tags()?;
        Ok(())
    }
}

impl LocalCreatePolicy {
    fn validate_tags(&self) -> Result<(), IamError> {
        let tags = self.tags();
        if tags.is_none() {
            return Ok(());
        }
        let tags = tags.unwrap();

        if tags.len() > constants::tag::SESSION_TAGS_MAX_COUNT {
            let error = ValidationError::too_many_tags(tags.len(), constants::tag::SESSION_TAGS_MAX_COUNT);
            return Err(IamError::new(IamErrorKind::InvalidInput, error.to_string().as_str(), self.aws_request_id()));
        }
        for (id, tag) in tags.iter().enumerate() {
            if let Err(error) = tag.validate(id + 1) {
                return Err(IamError::new(
                    IamErrorKind::InvalidInput,
                    error.to_string().as_str(),
                    self.aws_request_id(),
                ));
            }
        }
        Ok(())
    }

    fn validate_policy_name(&self) -> Result<(), IamError> {
        let policy_name = self.policy_name().unwrap_or(EMPTY_STR);
        if policy_name.trim().len() > 1 {
            return Ok(());
        }

        Ok(())
    }

    fn validate_policy_document(&self) -> Result<(), IamError> {
        Ok(())
    }
}
