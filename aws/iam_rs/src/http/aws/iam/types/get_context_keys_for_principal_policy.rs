use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct GetContextKeysForPrincipalPolicyRequest {
    #[serde(rename = "PolicySourceArn")]
    pub(crate) policy_source_arn: Option<types::arn_type::ArnType>,
    #[serde(rename = "PolicyInputList")]
    pub(crate) policy_input_list: Option<Vec<types::policy_document_type::PolicyDocumentType>>,
}

impl GetContextKeysForPrincipalPolicyRequest {
    pub(crate) fn policy_source_arn(&self) -> Option<&str> {
        self.policy_source_arn.as_deref()
    }
    pub(crate) fn policy_input_list(&self) -> Option<&[types::policy_document_type::PolicyDocumentType]> {
        self.policy_input_list.as_deref()
    }
}

impl validators::NamedValidator for &GetContextKeysForPrincipalPolicyRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_required(
            self.policy_source_arn(),
            format!("{at}.{}", "PolicySourceArn").as_str(),
        )?;
        validators::validate_named(
            self.policy_source_arn.as_ref(),
            format!("{at}.{}", "PolicySourceArn").as_str(),
        )?;
        if let Some(policy_input_list) = self.policy_input_list() {
            for (id, member) in policy_input_list.iter().enumerate() {
                validators::validate_named(
                    Some(member),
                    format!("{at}.{}.member.{id}", "PolicyInputList").as_str(),
                )?;
            }
        }
        Ok(())
    }
}
