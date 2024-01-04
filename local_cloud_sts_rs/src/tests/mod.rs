use aws_credential_types::provider::ProvideCredentials;

mod test_suite;

#[cfg(test)]
mod assume_role;

pub fn credentials_provider() -> impl ProvideCredentials {
    aws_credential_types::Credentials::new("access_key_id", "secret_access_key", None, None, "provider_name")
}
