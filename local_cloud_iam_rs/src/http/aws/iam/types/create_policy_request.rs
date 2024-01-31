use crate::http::aws::iam::types;
use crate::http::aws::iam::types::policy_document_type::PolicyDocumentType;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct CreatePolicyRequest {
    #[serde(rename = "Path")]
    pub(crate) path: Option<types::policy_path_type::PolicyPathType>,
    #[serde(rename = "Tags")]
    pub(crate) tags: Option<Vec<types::tag::Tag>>,
    #[serde(rename = "IsAttachable")]
    pub(crate) is_attachable: Option<local_cloud_common::types::Bool>,
    #[serde(rename = "PolicyName")]
    pub(crate) policy_name: Option<types::policy_name_type::PolicyNameType>,
    #[serde(rename = "PolicyDocument")]
    pub(crate) policy_document: Option<PolicyDocumentType>,
    #[serde(rename = "Description")]
    pub(crate) description: Option<types::policy_description_type::PolicyDescriptionType>,
}

impl CreatePolicyRequest {
    pub(crate) fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }
    pub(crate) fn tags(&self) -> Option<&[types::tag::Tag]> {
        self.tags.as_deref()
    }
    pub(crate) fn is_attachable(&self) -> Option<bool> {
        self.is_attachable.as_ref().map(|v| v.as_bool())
    }
    pub(crate) fn policy_name(&self) -> Option<&str> {
        self.policy_name.as_deref()
    }
    pub(crate) fn policy_document(&self) -> Option<&str> {
        // we expect that property is already validated, so, `unwrap` should be safe
        self.policy_document_type().map(|doc| doc.document().unwrap())
    }
    pub(crate) fn policy_document_type(&self) -> Option<&PolicyDocumentType> {
        self.policy_document.as_ref()
    }

    pub(crate) fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

impl local_cloud_validate::NamedValidator for &CreatePolicyRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_named(self.path.as_ref(), format!("{at}.{}", "Path").as_str())?;
        local_cloud_validate::validate_array_size_min(self.tags(), 0usize, format!("{at}.{}", "Tags").as_str())?;
        local_cloud_validate::validate_array_size_max(self.tags(), 50usize, format!("{at}.{}", "Tags").as_str())?;
        if let Some(tags) = self.tags() {
            for (id, member) in tags.iter().enumerate() {
                local_cloud_validate::validate_named(Some(member), format!("{at}.{}.member.{id}", "Tags").as_str())?;
            }
        }
        local_cloud_validate::validate_required(self.policy_name(), format!("{at}.{}", "PolicyName").as_str())?;
        local_cloud_validate::validate_named(self.policy_name.as_ref(), format!("{at}.{}", "PolicyName").as_str())?;
        local_cloud_validate::validate_required(
            self.policy_document_type(),
            format!("{at}.{}", "PolicyDocument").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.policy_document_type(),
            format!("{at}.{}", "PolicyDocument").as_str(),
        )?;
        local_cloud_validate::validate_named(self.description.as_ref(), format!("{at}.{}", "Description").as_str())?;
        Ok(())
    }
}
