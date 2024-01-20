use crate::http::aws::iam::types;
/**<p>A structure that represents user-provided metadata that can be associated with an IAM
      resource. For more information about tagging, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html">Tagging IAM resources</a> in the
      <i>IAM User Guide</i>.</p>*/
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct Tag {
    #[serde(rename = "Key")]
    pub(crate) key: Option<types::tag_key_type::TagKeyType>,
    #[serde(rename = "Value")]
    pub(crate) value: Option<types::tag_value_type::TagValueType>,
}
impl Tag {
    pub(crate) fn key(&self) -> Option<&str> {
        self.key.as_deref()
    }
    pub(crate) fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }
}
impl local_cloud_validate::NamedValidator for &Tag {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_required(
            self.key(),
            format!("{at}.{}", "Key").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.key.as_ref(),
            format!("{at}.{}", "Key").as_str(),
        )?;
        local_cloud_validate::validate_required(
            self.value(),
            format!("{at}.{}", "Value").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.value.as_ref(),
            format!("{at}.{}", "Value").as_str(),
        )?;
        Ok(())
    }
}
