use crate::http::aws::iam::types;
use crate::http::aws::iam::types::policy_document_type::PolicyDocumentType;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct PutUserPolicyRequest {
    #[serde(rename = "PolicyDocument")]
    pub(crate) policy_document: Option<PolicyDocumentType>,
    #[serde(rename = "PolicyName")]
    pub(crate) policy_name: Option<types::policy_name_type::PolicyNameType>,
    #[serde(rename = "UserName")]
    pub(crate) user_name: Option<types::existing_user_name_type::ExistingUserNameType>,
}

impl PutUserPolicyRequest {
    pub(crate) fn policy_name(&self) -> Option<&str> {
        self.policy_name.as_deref()
    }
    pub(crate) fn user_name(&self) -> Option<&str> {
        self.user_name.as_deref()
    }
    pub(crate) fn policy_document(&self) -> Option<&str> {
        // we expect that property is already validated, so, `unwrap` should be safe
        self.policy_document_type().map(|doc| doc.document().unwrap())
    }
    pub(crate) fn policy_document_type(&self) -> Option<&PolicyDocumentType> {
        self.policy_document.as_ref()
    }
}

impl validators::NamedValidator for &PutUserPolicyRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_required(
            self.policy_document_type(),
            format!("{at}.{}", "PolicyDocument").as_str(),
        )?;
        validators::validate_named(
            self.policy_document_type(),
            format!("{at}.{}", "PolicyDocument").as_str(),
        )?;
        validators::validate_required(self.policy_name(), format!("{at}.{}", "PolicyName").as_str())?;
        validators::validate_named(self.policy_name.as_ref(), format!("{at}.{}", "PolicyName").as_str())?;
        validators::validate_required(self.user_name(), format!("{at}.{}", "UserName").as_str())?;
        validators::validate_named(self.user_name.as_ref(), format!("{at}.{}", "UserName").as_str())?;
        Ok(())
    }
}
