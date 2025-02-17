use std::ops::Deref;

/**<p>Contains a thumbprint for an identity provider's server certificate.</p>
<p>The identity provider's server certificate thumbprint is the hex-encoded SHA-1 hash
value of the self-signed X.509 certificate. This thumbprint is used by the domain where the
OpenID Connect provider makes its keys available. The thumbprint is always a 40-character
string.</p>*/
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct ThumbprintType(String);

impl Deref for ThumbprintType {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl validators::NamedValidator for &ThumbprintType {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_str_length_min(Some(&self), 40usize, at)?;
        validators::validate_str_length_max(Some(&self), 40usize, at)?;
        Ok(())
    }
}
