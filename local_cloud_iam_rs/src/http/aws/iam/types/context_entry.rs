use crate::http::aws::iam::types;
/**<p>Contains information about a condition context key. It includes the name of the key and
specifies the value (or values, if the context key supports multiple values) to use in the
simulation. This information is used when evaluating the <code>Condition</code> elements of
the input policies.</p>
<p>This data type is used as an input parameter to <a>SimulateCustomPolicy</a>
and <a>SimulatePrincipalPolicy</a>.</p>*/
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct ContextEntry {
    #[serde(rename = "ContextKeyName")]
    pub(crate) context_key_name: Option<types::context_key_name_type::ContextKeyNameType>,
    #[serde(rename = "ContextKeyValues")]
    pub(crate) context_key_values: Option<Vec<types::context_key_value_type::ContextKeyValueType>>,
    #[serde(rename = "ContextKeyType")]
    pub(crate) context_key_type: Option<types::context_key_type_enum::ContextKeyTypeEnum>,
}
impl ContextEntry {
    pub(crate) fn context_key_name(&self) -> Option<&str> {
        self.context_key_name.as_deref()
    }
    pub(crate) fn context_key_values(&self) -> Option<&[types::context_key_value_type::ContextKeyValueType]> {
        self.context_key_values.as_deref()
    }
    pub(crate) fn context_key_type(&self) -> Option<&types::context_key_type_enum::ContextKeyTypeEnum> {
        self.context_key_type.as_ref()
    }
}
impl local_cloud_validate::NamedValidator for &ContextEntry {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_named(
            self.context_key_name.as_ref(),
            format!("{at}.{}", "ContextKeyName").as_str(),
        )?;
        if let Some(context_key_values) = self.context_key_values() {
            for (id, member) in context_key_values.iter().enumerate() {
                local_cloud_validate::validate_named(
                    Some(member),
                    format!("{at}.{}.member.{id}", "ContextKeyValues").as_str(),
                )?;
            }
        }
        Ok(())
    }
}
