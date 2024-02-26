use local_cloud_validate::{validate_named, validate_required};

use crate::http::aws::iam::types;
use crate::http::aws::iam::types::policy_document_type::PolicyDocumentType;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct SimulateCustomPolicyRequest {
    #[serde(rename = "ActionNames")]
    pub(crate) action_names: Option<Vec<types::action_name_type::ActionNameType>>,
    #[serde(rename = "PolicyInputList")]
    pub(crate) policy_input_list: Option<Vec<PolicyDocumentType>>,
    #[serde(rename = "ResourcePolicy")]
    pub(crate) resource_policy: Option<PolicyDocumentType>,
    #[serde(rename = "CallerArn")]
    pub(crate) caller_arn: Option<types::resource_name_type::ResourceNameType>,
    #[serde(rename = "ResourceOwner")]
    pub(crate) resource_owner: Option<types::resource_name_type::ResourceNameType>,
    #[serde(rename = "ResourceHandlingOption")]
    pub(crate) resource_handling_option: Option<types::resource_handling_option_type::ResourceHandlingOptionType>,
    #[serde(rename = "MaxItems")]
    pub(crate) max_items: Option<types::max_items_type::MaxItemsType>,
    #[serde(rename = "ContextEntries")]
    pub(crate) context_entries: Option<Vec<types::context_entry::ContextEntry>>,
    #[serde(rename = "PermissionsBoundaryPolicyInputList")]
    pub(crate) permissions_boundary_policy_input_list: Option<Vec<PolicyDocumentType>>,
    #[serde(rename = "Marker")]
    pub(crate) marker: Option<types::marker_type::MarkerType>,
    #[serde(rename = "ResourceArns")]
    pub(crate) resource_arns: Option<Vec<types::resource_name_type::ResourceNameType>>,
}

impl SimulateCustomPolicyRequest {
    pub(crate) fn action_names(&self) -> Option<&[types::action_name_type::ActionNameType]> {
        self.action_names.as_deref()
    }
    pub(crate) fn policy_input_list(&self) -> Option<&[PolicyDocumentType]> {
        self.policy_input_list.as_deref()
    }
    pub(crate) fn resource_policy(&self) -> Option<&str> {
        // we expect that property is already validated, so, `unwrap` should be safe
        self.resource_policy_type().map(|doc| doc.document().unwrap())
    }
    pub(crate) fn resource_policy_type(&self) -> Option<&PolicyDocumentType> {
        self.resource_policy.as_ref()
    }
    pub(crate) fn caller_arn(&self) -> Option<&str> {
        self.caller_arn.as_deref()
    }
    pub(crate) fn resource_owner(&self) -> Option<&str> {
        self.resource_owner.as_deref()
    }
    pub(crate) fn resource_handling_option(&self) -> Option<&str> {
        self.resource_handling_option.as_deref()
    }
    pub(crate) fn max_items(&self) -> Option<&i32> {
        self.max_items.as_deref()
    }
    pub(crate) fn context_entries(&self) -> Option<&[types::context_entry::ContextEntry]> {
        self.context_entries.as_deref()
    }
    pub(crate) fn permissions_boundary_policy_input_list(&self) -> Option<&[PolicyDocumentType]> {
        self.permissions_boundary_policy_input_list.as_deref()
    }
    pub(crate) fn marker(&self) -> Option<&str> {
        self.marker.as_deref()
    }
    pub(crate) fn resource_arns(&self) -> Option<&[types::resource_name_type::ResourceNameType]> {
        self.resource_arns.as_deref()
    }
}

impl local_cloud_validate::NamedValidator for &SimulateCustomPolicyRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        validate_required(self.action_names(), format!("{at}.{}", "ActionNames").as_str())?;
        if let Some(action_names) = self.action_names() {
            for (id, member) in action_names.iter().enumerate() {
                validate_named(Some(member), format!("{at}.{}.member.{id}", "ActionNames").as_str())?;
            }
        }
        validate_required(self.policy_input_list(), format!("{at}.{}", "PolicyInputList").as_str())?;
        if let Some(policy_input_list) = self.policy_input_list() {
            for (id, member) in policy_input_list.iter().enumerate() {
                validate_named(Some(member), format!("{at}.{}.member.{id}", "PolicyInputList").as_str())?;
            }
        }
        validate_named(self.resource_policy_type(), format!("{at}.{}", "ResourcePolicy").as_str())?;
        validate_named(self.caller_arn.as_ref(), format!("{at}.{}", "CallerArn").as_str())?;
        validate_named(self.resource_owner.as_ref(), format!("{at}.{}", "ResourceOwner").as_str())?;
        validate_named(self.resource_handling_option.as_ref(), format!("{at}.{}", "ResourceHandlingOption").as_str())?;
        validate_named(self.max_items.as_ref(), format!("{at}.{}", "MaxItems").as_str())?;
        if let Some(context_entries) = self.context_entries() {
            for (id, member) in context_entries.iter().enumerate() {
                validate_named(Some(member), format!("{at}.{}.member.{id}", "ContextEntries").as_str())?;
            }
        }
        if let Some(permissions_boundary_policy_input_list) = self.permissions_boundary_policy_input_list() {
            for (id, member) in permissions_boundary_policy_input_list.iter().enumerate() {
                validate_named(
                    Some(member),
                    format!("{at}.{}.member.{id}", "PermissionsBoundaryPolicyInputList").as_str(),
                )?;
            }
        }
        validate_named(self.marker.as_ref(), format!("{at}.{}", "Marker").as_str())?;
        if let Some(resource_arns) = self.resource_arns() {
            for (id, member) in resource_arns.iter().enumerate() {
                validate_named(Some(member), format!("{at}.{}.member.{id}", "ResourceArns").as_str())?;
            }
        }
        Ok(())
    }
}
