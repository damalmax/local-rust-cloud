use local_cloud_iam_policy_document::types::LocalPolicyDocument;

use crate::http::aws::iam::actions::create_policy::LocalCreatePolicy;
use crate::http::aws::iam::actions::error::{IamError, IamErrorKind};
use crate::http::aws::iam::actions::validate::{IamValidator, ValidationError};
use crate::http::aws::iam::constants;

const EMPTY_STR: &str = "";

impl IamValidator for LocalCreatePolicy {
    fn validate(&self, aws_request_id: &str) -> Result<(), IamError> {
        self.validate_policy_name(aws_request_id)?;
        self.validate_policy_document(aws_request_id)?;
        self.validate_tags(aws_request_id)?;
        Ok(())
    }
}

impl LocalCreatePolicy {
    fn validate_tags(&self, aws_request_id: &str) -> Result<(), IamError> {
        let tags = self.tags();
        if tags.is_none() {
            return Ok(());
        }
        let tags = tags.unwrap();

        if tags.len() > constants::tag::SESSION_TAGS_MAX_COUNT {
            let error = ValidationError::too_many_tags(tags.len(), constants::tag::SESSION_TAGS_MAX_COUNT);
            return Err(IamError::new(IamErrorKind::InvalidInput, error.to_string().as_str(), aws_request_id));
        }
        for (id, tag) in tags.iter().enumerate() {
            if let Err(error) = tag.validate(id + 1) {
                return Err(IamError::new(IamErrorKind::InvalidInput, error.to_string().as_str(), aws_request_id));
            }
        }
        Ok(())
    }

    fn validate_policy_name(&self, _aws_request_id: &str) -> Result<(), IamError> {
        let policy_name = self.policy_name().unwrap_or(EMPTY_STR);
        if policy_name.trim().len() > 1 {
            return Ok(());
        }

        Ok(())
    }

    fn validate_policy_document(&self, aws_request_id: &str) -> Result<(), IamError> {
        if self.policy_document().is_none() {
            Ok(())
        } else {
            let policy_document: LocalPolicyDocument =
                serde_json::from_str(self.policy_document().unwrap()).map_err(|_err| {
                    IamError::new(IamErrorKind::MalformedPolicyDocument, "Malformed policy document.", aws_request_id)
                })?;
            let json = serde_json::to_string(&policy_document).map_err(|_err| {
                IamError::new(IamErrorKind::ServiceFailure, "Failed to analyze Policy Document.", aws_request_id)
            })?;

            if json.chars().count() > constants::policy::MANAGED_POLICY_MAX_SIZE {
                return Err(IamError::new(IamErrorKind::InvalidInput, "Policy size", aws_request_id));
            }
            Ok(())
        }
    }
}
