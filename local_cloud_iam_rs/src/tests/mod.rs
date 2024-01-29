use aws_config::BehaviorVersion;
use aws_credential_types::provider::{ProvideCredentials, SharedCredentialsProvider};
use aws_types::region::Region;
use aws_types::SdkConfig;

mod test_suite;

#[cfg(test)]
mod add_user_to_group;
#[cfg(test)]
mod attach_group_policy;
#[cfg(test)]
mod create_group;
#[cfg(test)]
mod create_policy;
#[cfg(test)]
mod create_policy_version;
#[cfg(test)]
mod create_role;
#[cfg(test)]
mod create_user;
pub mod fixture;
#[cfg(test)]
mod get_group;
#[cfg(test)]
mod list_groups;
#[cfg(test)]
mod list_policies;

pub fn credentials_provider() -> impl ProvideCredentials {
    aws_credential_types::Credentials::new("AKIAIOSFODNN201ADMIN", "secret_access_key", None, None, "provider_name")
}

pub fn aws_config(port: u16) -> SdkConfig {
    SdkConfig::builder()
        .region(Some(Region::new("eu-local-1")))
        .endpoint_url(format!("http://localhost:{}/iam", port))
        .credentials_provider(SharedCredentialsProvider::new(credentials_provider()))
        .behavior_version(BehaviorVersion::latest())
        .build()
}
