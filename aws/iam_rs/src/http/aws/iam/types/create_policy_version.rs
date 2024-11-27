use crate::http::aws::iam::types;
use crate::http::aws::iam::types::policy_document_type::PolicyDocumentType;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct CreatePolicyVersionRequest {
    #[serde(rename = "SetAsDefault")]
    pub(crate) set_as_default: Option<utils::types::Bool>,
    #[serde(rename = "PolicyDocument")]
    pub(crate) policy_document: Option<PolicyDocumentType>,
    #[serde(rename = "PolicyArn")]
    pub(crate) policy_arn: Option<types::arn_type::ArnType>,
}

impl CreatePolicyVersionRequest {
    pub(crate) fn set_as_default(&self) -> Option<bool> {
        self.set_as_default.as_ref().map(|v| v.as_bool())
    }
    pub(crate) fn policy_document(&self) -> Option<&str> {
        // we expect that property is already validated, so, `unwrap` should be safe
        self.policy_document_type().map(|doc| doc.document().unwrap())
    }
    pub(crate) fn policy_document_type(&self) -> Option<&PolicyDocumentType> {
        self.policy_document.as_ref()
    }
    pub(crate) fn policy_arn(&self) -> Option<&str> {
        self.policy_arn.as_deref()
    }
}

impl validators::NamedValidator for &CreatePolicyVersionRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_required(
            self.policy_document_type(),
            format!("{at}.{}", "PolicyDocument").as_str(),
        )?;
        validators::validate_named(
            self.policy_document_type(),
            format!("{at}.{}", "PolicyDocument").as_str(),
        )?;
        validators::validate_required(self.policy_arn(), format!("{at}.{}", "PolicyArn").as_str())?;
        validators::validate_named(self.policy_arn.as_ref(), format!("{at}.{}", "PolicyArn").as_str())?;
        Ok(())
    }
}
