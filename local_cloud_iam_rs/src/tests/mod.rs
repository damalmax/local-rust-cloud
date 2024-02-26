use aws_config::BehaviorVersion;
use aws_credential_types::provider::{ProvideCredentials, SharedCredentialsProvider};
use aws_types::region::Region;
use aws_types::SdkConfig;

mod test_suite;

mod add_client_id_to_open_id_connect_provider;
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
mod create_saml_provider;
mod create_user;
mod create_virtual_mfa_device;
mod enable_mfa_device;
pub mod fixture;
mod get_group;
mod get_group_policy;
mod get_mfa_device;
mod get_role_policy;
mod get_user_policy;
mod list_group_policies;
mod list_groups;
mod list_groups_for_user;
mod list_instance_profile_tags;
mod list_mfa_device_tags;
mod list_open_id_connect_provider_tags;
mod list_policies;
mod list_policy_tags;
mod list_policy_versions;
mod list_role_policies;
mod list_role_tags;
mod list_roles;
mod list_saml_provider_tags;
mod list_user_policies;
mod list_user_tags;
mod list_users;
mod list_virtual_mfa_devices;
mod put_group_policy;
mod put_role_policy;
mod put_user_policy;
mod tag_instance_profile;
mod tag_mfa_device;
mod tag_open_id_connect_provider;
mod tag_policy;
mod tag_role;
mod tag_saml_provider;
mod tag_server_certificate;
mod tag_user;
mod untag_instance_profile;
mod untag_mfa_device;
mod untag_open_id_connect_provider;
mod untag_policy;
mod untag_role;
mod untag_saml_provider;
mod untag_server_certificate;
mod untag_user;
mod update_ssh_public_key;
mod update_user;
mod upload_server_certificate;
mod upload_signing_certificate;
mod upload_ssh_public_key;

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
