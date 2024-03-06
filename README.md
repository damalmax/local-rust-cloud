# Local Rust Cloud DEV

The project is created to help with serverless apps testing on local environment.

**NOTE:** it is not ready for use yet.

# Services supported

## AWS

| Service Name | Binary name               | Depends On |
|--------------|---------------------------|------------|
| IAM          | `local_rust_cloud_iam_rs` |            |
| STS          | `local_rust_cloud_sts_rs` |            |

### Services implementation coverage

<details>
<summary>1. IAM</summary>

- [X] add_client_id_to_open_id_connect_provider
- [X] add_role_to_instance_profile
- [X] add_user_to_group
- [X] attach_group_policy
- [X] attach_role_policy
- [X] attach_user_policy
- [ ] change_password
- [ ] create_access_key
- [ ] create_account_alias
- [X] create_group
- [X] create_instance_profile
- [X] create_login_profile
- [X] create_open_id_connect_provider
- [X] create_policy
- [X] create_policy_version
- [X] create_role
- [X] create_saml_provider
- [ ] create_service_linked_role
- [ ] create_service_specific_credential
- [X] create_user
- [X] create_virtual_mfa_device
- [X] deactivate_mfa_device
- [ ] delete_access_key
- [ ] delete_account_alias
- [ ] delete_account_password_policy
- [ ] delete_group
- [X] delete_group_policy
- [ ] delete_instance_profile
- [ ] delete_login_profile
- [ ] delete_open_id_connect_provider
- [ ] delete_policy
- [X] delete_policy_version
- [ ] delete_role
- [ ] delete_role_permissions_boundary
- [X] delete_role_policy
- [ ] delete_saml_provider
- [ ] delete_server_certificate
- [ ] delete_service_linked_role
- [ ] delete_service_specific_credential
- [ ] delete_signing_certificate
- [ ] delete_ssh_public_key
- [ ] delete_user
- [X] delete_user_permissions_boundary
- [X] delete_user_policy
- [ ] delete_virtual_mfa_device
- [X] detach_group_policy
- [X] detach_role_policy
- [X] detach_user_policy
- [X] enable_mfa_device
- [ ] generate_credential_report
- [ ] generate_organizations_access_report
- [ ] generate_service_last_accessed_details
- [ ] get_access_key_last_used
- [ ] get_account_authorization_details
- [ ] get_account_password_policy
- [ ] get_account_summary
- [ ] get_context_keys_for_custom_policy
- [ ] get_context_keys_for_principal_policy
- [ ] get_credential_report
- [X] get_group
- [X] get_group_policy
- [ ] get_instance_profile
- [ ] get_login_profile
- [X] get_mfa_device
- [ ] get_open_id_connect_provider
- [ ] get_organizations_access_report
- [X] get_policy
- [X] get_policy_version
- [X] get_role
- [X] get_role_policy
- [X] get_saml_provider
- [ ] get_server_certificate
- [ ] get_service_last_accessed_details
- [ ] get_service_last_accessed_details_with_entities
- [ ] get_service_linked_role_deletion_status
- [ ] get_ssh_public_key
- [ ] get_user
- [X] get_user_policy
- [ ] list_access_keys
- [ ] list_account_aliases
- [ ] list_attached_group_policies
- [ ] list_attached_role_policies
- [ ] list_attached_user_policies
- [ ] list_entities_for_policy
- [X] list_group_policies
- [X] list_groups
- [X] list_groups_for_user
- [X] list_instance_profile_tags
- [X] list_instance_profiles
- [ ] list_instance_profiles_for_role
- [X] list_mfa_device_tags
- [ ] list_mfa_devices
- [X] list_open_id_connect_provider_tags
- [X] list_open_id_connect_providers
- [X] list_policies
- [ ] list_policies_granting_service_access
- [X] list_policy_tags
- [X] list_policy_versions
- [X] list_role_policies
- [X] list_role_tags
- [X] list_roles
- [X] list_saml_provider_tags
- [X] list_saml_providers
- [X] list_server_certificate_tags
- [X] list_server_certificates
- [ ] list_service_specific_credentials
- [X] list_signing_certificates
- [X] list_ssh_public_keys
- [X] list_user_policies
- [X] list_user_tags
- [X] list_users
- [X] list_virtual_mfa_devices
- [X] put_group_policy
- [X] put_role_permissions_boundary
- [X] put_role_policy
- [X] put_user_permissions_boundary
- [X] put_user_policy
- [ ] remove_client_id_from_open_id_connect_provider
- [ ] remove_role_from_instance_profile
- [ ] remove_user_from_group
- [ ] reset_service_specific_credential
- [ ] resync_mfa_device
- [X] set_default_policy_version
- [ ] set_security_token_service_preferences
- [ ] simulate_custom_policy
- [ ] simulate_principal_policy
- [X] tag_instance_profile
- [X] tag_mfa_device
- [X] tag_open_id_connect_provider
- [X] tag_policy
- [X] tag_role
- [X] tag_saml_provider
- [X] tag_server_certificate
- [X] tag_user
- [X] untag_instance_profile
- [X] untag_mfa_device
- [X] untag_open_id_connect_provider
- [X] untag_policy
- [X] untag_role
- [X] untag_saml_provider
- [X] untag_server_certificate
- [X] untag_user
- [ ] update_access_key
- [ ] update_account_password_policy
- [ ] update_assume_role_policy
- [X] update_group
- [ ] update_login_profile
- [ ] update_open_id_connect_provider_thumbprint
- [ ] update_role
- [ ] update_role_description
- [X] update_saml_provider
- [X] update_server_certificate
- [ ] update_service_specific_credential
- [X] update_signing_certificate
- [X] update_ssh_public_key
- [X] update_user
- [X] upload_server_certificate
- [X] upload_signing_certificate
- [X] upload_ssh_public_key

</details>

<details>
<summary>2. STS</summary>

- [X] assume_role
- [ ] assume_role_with_saml
- [ ] assume_role_with_web_identity
- [ ] get_access_key_info
- [ ] get_caller_identity
- [ ] get_federation_token
- [ ] get_session_token

</details>

# How to contribute

All services are implemented using [Rust language](https://www.rust-lang.org/).

# How to run

## Setup `etcd`

The project requires a working etcd cluster. Below you can find a way how to setup the cluster with Docker Compose. The
Docker Compose configuration file could be found in the `docker` folder:

```bash
$ cd docker && docker compose up -d
```

The command above will create a new etcd cluster which consists of 3 instances:

- 127.0.0.1:2379
- 127.0.0.1:2479
- 127.0.0.1:2579

## Setup `.env` file

The `.env` file is the main configuration file if you run Local Rust Cloud applications from the Github repository.

# Third-Party Software used

- [SQLite DB](https://www.sqlite.org/index.html) - for storing all services-related data.
- [etcd](https://etcd.io/) - for storing app configs, shared data, etc.
- [Docker](https://www.docker.com) - for applications containerization and some tasks execution.
- [Protobuf compiler](https://grpc.io/docs/protoc-installation/) - Dependency required for project compilation.

# Windows Build prerequisites

Guide Used: https://gist.github.com/zeljic/d8b542788b225b1bcb5fce169ee28c55

### How to build SQLite3 .lib file on Windows 10

1. Download source from [source](https://www.sqlite.org/download.html)

For example: [source](https://www.sqlite.org/2022/sqlite-amalgamation-3390300.zip)

2. Download binary from [binary](https://www.sqlite.org/download.html)

For example: [binary](https://www.sqlite.org/2022/sqlite-dll-win64-x64-3390300.zip)

3. Extract both archives to the same directory

4. Open **Developer Command Prompt for VS 2017** by typing *Developer Command* in Windows Search

5. Go to directory where you've extracted **source code** and **binary** files (via opened cmd)
6. Run
   ```lib /DEF:sqlite3.def /OUT:sqlite3.lib /MACHINE:x64```
