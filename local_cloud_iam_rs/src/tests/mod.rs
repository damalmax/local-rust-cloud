use aws_config::BehaviorVersion;
use aws_credential_types::provider::{ProvideCredentials, SharedCredentialsProvider};
use aws_types::region::Region;
use aws_types::SdkConfig;

mod test_suite;

mod add_role_to_instance_profile;
mod add_user_to_group;
mod attach_group_policy;
mod attach_role_policy;
mod attach_user_policy;
mod create_group;
mod create_instance_profile;
mod create_login_profile;
mod create_open_id_connect_provider;
mod create_policy;
mod create_policy_version;
mod create_role;
mod create_user;
pub mod fixture;
mod get_group;
mod list_groups;
mod list_policies;
mod list_policy_tags;
mod list_role_tags;
mod list_roles;
mod list_user_tags;
mod list_users;
mod tag_instance_profile;
mod tag_policy;
mod tag_role;
mod tag_user;

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
