use serde::Deserialize;

use local_cloud_validate::{validate_array_size_max, validate_named, validate_required, ValidationError, Validator};

use crate::http::aws::iam::actions::types::path::PathType;
use crate::http::aws::iam::actions::types::policy_description::PolicyDescriptionType;
use crate::http::aws::iam::actions::types::policy_name::PolicyNameType;
use crate::http::aws::iam::actions::types::tag::TagType;
use crate::http::aws::iam::constants;

#[derive(Debug, Deserialize)]
pub(crate) struct CreatePolicyType {
    #[serde(rename = "PolicyName")]
    pub(crate) policy_name: Option<PolicyNameType>,
    #[serde(rename = "Path")]
    pub(crate) path: Option<PathType>,
    #[serde(rename = "PolicyDocument")]
    pub(crate) policy_document: Option<String>,
    #[serde(rename = "Description")]
    pub(crate) description: Option<PolicyDescriptionType>,
    #[serde(rename = "Tags")]
    pub(crate) tags: Option<Vec<TagType>>,
    #[serde(rename = "IsAttachable")]
    pub(crate) attachable: Option<bool>,
}

impl CreatePolicyType {
    pub fn policy_name(&self) -> Option<&str> {
        self.policy_name.as_deref()
    }

    pub fn policy_document(&self) -> Option<&str> {
        self.policy_document.as_deref()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    pub fn tags(&self) -> Option<&[TagType]> {
        self.tags.as_deref()
    }

    pub fn attachable(&self) -> Option<bool> {
        self.attachable
    }
}

impl Validator for CreatePolicyType {
    fn validate(&self) -> Result<(), ValidationError> {
        validate_required(self.policy_name(), "PolicyName")?;
        validate_named(self.policy_name.as_ref(), "PolicyName")?;
        validate_required(self.policy_document(), "PolicyDocument")?;
        validate_named(self.path.as_ref(), "Path")?;
        validate_named(self.description.as_ref(), "Description")?;
        validate_array_size_max(self.tags(), constants::tag::SESSION_TAGS_MAX_COUNT, "Tags")?;

        if let Some(tags) = self.tags() {
            for (id, tag) in tags.iter().enumerate() {
                validate_named(Some(tag), format!("Tags.member.{id}").as_str())?;
            }
        }
        Ok(())
    }
}
