use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct TagPolicyRequest {
    #[serde(rename = "PolicyArn")]
    pub(crate) policy_arn: Option<types::arn_type::ArnType>,
    #[serde(rename = "Tags")]
    pub(crate) tags: Option<Vec<types::tag::Tag>>,
}

impl TagPolicyRequest {
    pub(crate) fn policy_arn(&self) -> Option<&str> {
        self.policy_arn.as_deref()
    }
    pub(crate) fn tags(&self) -> Option<&[types::tag::Tag]> {
        self.tags.as_deref()
    }
}

impl validators::NamedValidator for &TagPolicyRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_required(self.policy_arn(), format!("{at}.{}", "PolicyArn").as_str())?;
        validators::validate_named(self.policy_arn.as_ref(), format!("{at}.{}", "PolicyArn").as_str())?;
        validators::validate_required(self.tags(), format!("{at}.{}", "Tags").as_str())?;
        validators::validate_array_size_min(self.tags(), 0usize, format!("{at}.{}", "Tags").as_str())?;
        validators::validate_array_size_max(self.tags(), 50usize, format!("{at}.{}", "Tags").as_str())?;
        if let Some(tags) = self.tags() {
            for (id, member) in tags.iter().enumerate() {
                validators::validate_named(Some(member), format!("{at}.{}.member.{id}", "Tags").as_str())?;
            }
        }
        Ok(())
    }
}
