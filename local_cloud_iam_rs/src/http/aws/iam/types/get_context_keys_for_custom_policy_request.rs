use crate::http::aws::iam::types;
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct GetContextKeysForCustomPolicyRequest {
    #[serde(rename = "PolicyInputList")]
    pub(crate) policy_input_list: Option<Vec<types::policy_document_type::PolicyDocumentType>>,
}
impl GetContextKeysForCustomPolicyRequest {
    pub(crate) fn policy_input_list(&self) -> Option<&[types::policy_document_type::PolicyDocumentType]> {
        self.policy_input_list.as_deref()
    }
}
impl local_cloud_validate::NamedValidator for &GetContextKeysForCustomPolicyRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_required(
            self.policy_input_list(),
            format!("{at}.{}", "PolicyInputList").as_str(),
        )?;
        if let Some(policy_input_list) = self.policy_input_list() {
            for (id, member) in policy_input_list.iter().enumerate() {
                local_cloud_validate::validate_named(
                    Some(member),
                    format!("{at}.{}.member.{id}", "PolicyInputList").as_str(),
                )?;
            }
        }
        Ok(())
    }
}
