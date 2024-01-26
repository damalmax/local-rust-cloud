use std::ops::Deref;

/**<p>The Amazon Resource Name (ARN). ARNs are unique identifiers for Amazon Web Services resources.</p>
<p>For more information about ARNs, go to <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in
the <i>Amazon Web Services General Reference</i>. </p>*/
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct ArnType(String);

impl Deref for ArnType {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl local_cloud_validate::NamedValidator for &ArnType {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_str_length_min(Some(&self), 20usize, at)?;
        local_cloud_validate::validate_str_length_max(Some(&self), 2048usize, at)?;
        Ok(())
    }
}
