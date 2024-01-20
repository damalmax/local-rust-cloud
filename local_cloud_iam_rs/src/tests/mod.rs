use aws_credential_types::provider::ProvideCredentials;

mod test_suite;

#[cfg(test)]
mod create_policy;
#[cfg(test)]
mod create_policy_version;
#[cfg(test)]
mod create_user;

pub fn credentials_provider() -> impl ProvideCredentials {
    aws_credential_types::Credentials::new("AKIAIOSFODNN201ADMIN", "secret_access_key", None, None, "provider_name")
}
