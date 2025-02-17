use validators::{validate_named, validate_required};

use crate::http::aws::iam::types;
use crate::http::aws::iam::types::policy_document_type::PolicyDocumentType;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct PutGroupPolicyRequest {
    #[serde(rename = "PolicyName")]
    pub(crate) policy_name: Option<types::policy_name_type::PolicyNameType>,
    #[serde(rename = "GroupName")]
    pub(crate) group_name: Option<types::group_name_type::GroupNameType>,
    #[serde(rename = "PolicyDocument")]
    pub(crate) policy_document: Option<PolicyDocumentType>,
}

impl PutGroupPolicyRequest {
    pub(crate) fn policy_name(&self) -> Option<&str> {
        self.policy_name.as_deref()
    }
    pub(crate) fn group_name(&self) -> Option<&str> {
        self.group_name.as_deref()
    }
    pub(crate) fn policy_document(&self) -> Option<&str> {
        // we expect that property is already validated, so, `unwrap` should be safe
        self.policy_document_type().map(|doc| doc.document().unwrap())
    }
    pub(crate) fn policy_document_type(&self) -> Option<&PolicyDocumentType> {
        self.policy_document.as_ref()
    }
}

impl validators::NamedValidator for &PutGroupPolicyRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validate_required(self.policy_name(), format!("{at}.{}", "PolicyName").as_str())?;
        validate_named(self.policy_name.as_ref(), format!("{at}.{}", "PolicyName").as_str())?;
        validate_required(self.group_name(), format!("{at}.{}", "GroupName").as_str())?;
        validate_named(self.group_name.as_ref(), format!("{at}.{}", "GroupName").as_str())?;
        validate_required(self.policy_document_type(), format!("{at}.{}", "PolicyDocument").as_str())?;
        validate_named(self.policy_document_type(), format!("{at}.{}", "PolicyDocument").as_str())?;
        Ok(())
    }
}
