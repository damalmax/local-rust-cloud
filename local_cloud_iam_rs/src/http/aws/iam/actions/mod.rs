pub(crate) mod action;
pub(crate) mod create_group;
pub(crate) mod create_instance_profile;
pub(crate) mod create_login_profile;
pub(crate) mod create_open_id_connect_provider;
pub(crate) mod create_policy;
pub(crate) mod create_policy_version;
pub(crate) mod create_role;
pub(crate) mod create_saml_provider;
pub(crate) mod create_user;
pub(crate) mod create_virtual_mfa_device;
pub(crate) mod error;

macro_rules! action {
    ($entity:ident, $operation:ident, $mod_request:ident, $request:ident, $response:ident) => {
        mod $operation {
            use aws_sdk_iam::operation::$operation::$response;

            use local_cloud_db::LocalDb;

            use crate::http::aws::iam;
            use crate::http::aws::iam::actions::action::Action;
            use crate::http::aws::iam::actions::error::ApiError;
            use crate::http::aws::iam::operations::ctx::OperationCtx;
            use crate::http::aws::iam::operations::error::OperationError;
            use crate::http::aws::iam::outputs::wrapper::OutputWrapper;
            use crate::http::aws::iam::types::$mod_request::$request;

            impl Action for $request {
                type Output = OutputWrapper<$response>;
                async fn execute(
                    &self, account_id: i64, aws_request_id: &str, db: &LocalDb,
                ) -> Result<Self::Output, ApiError> {
                    let ctx = OperationCtx::new(account_id, aws_request_id);
                    let output = iam::operations::$entity::$operation(&ctx, self, db)
                        .await
                        .map_err(|error| match error {
                            OperationError::Service { kind, msg } => ApiError::new(kind, &msg, aws_request_id),
                            OperationError::Validation(error) => {
                                ApiError::from_validation_error(&error, aws_request_id)
                            }
                        })?;

                    Ok(OutputWrapper::new(output, aws_request_id))
                }
            }
        }
    };
}

action!(
    instance_profile,
    add_role_to_instance_profile,
    add_role_to_instance_profile,
    AddRoleToInstanceProfileRequest,
    AddRoleToInstanceProfileOutput
);
action!(
    instance_profile,
    tag_instance_profile,
    tag_instance_profile,
    TagInstanceProfileRequest,
    TagInstanceProfileOutput
);
action!(
    instance_profile,
    list_instance_profile_tags,
    list_instance_profile_tags,
    ListInstanceProfileTagsRequest,
    ListInstanceProfileTagsOutput
);
action!(
    instance_profile,
    untag_instance_profile,
    untag_instance_profile,
    UntagInstanceProfileRequest,
    UntagInstanceProfileOutput
);
action!(group, add_user_to_group, add_user_to_group, AddUserToGroupRequest, AddUserToGroupOutput);
action!(group, attach_group_policy, attach_group_policy, AttachGroupPolicyRequest, AttachGroupPolicyOutput);
action!(group, get_group, get_group, GetGroupRequest, GetGroupOutput);
action!(group, get_group_policy, get_group_policy, GetGroupPolicyRequest, GetGroupPolicyOutput);
action!(group, list_groups, list_groups, ListGroupsRequest, ListGroupsOutput);
action!(group, list_group_policies, list_group_policies, ListGroupPoliciesRequest, ListGroupPoliciesOutput);
action!(group, list_groups_for_user, list_groups_for_user, ListGroupsForUserRequest, ListGroupsForUserOutput);
action!(group, put_group_policy, put_group_policy, PutGroupPolicyRequest, PutGroupPolicyOutput);
action!(policy, list_policies, list_policies, ListPoliciesRequest, ListPoliciesOutput);
action!(policy, list_policy_versions, list_policy_versions, ListPolicyVersionsRequest, ListPolicyVersionsOutput);
action!(policy, list_policy_tags, list_policy_tags, ListPolicyTagsRequest, ListPolicyTagsOutput);
action!(policy, tag_policy, tag_policy, TagPolicyRequest, TagPolicyOutput);
action!(policy, untag_policy, untag_policy, UntagPolicyRequest, UntagPolicyOutput);
action!(user, attach_user_policy, attach_user_policy, AttachUserPolicyRequest, AttachUserPolicyOutput);
action!(user, get_user_policy, get_user_policy, GetUserPolicyRequest, GetUserPolicyOutput);
action!(user, list_users, list_users, ListUsersRequest, ListUsersOutput);
action!(user, list_user_policies, list_user_policies, ListUserPoliciesRequest, ListUserPoliciesOutput);
action!(user, list_user_tags, list_user_tags, ListUserTagsRequest, ListUserTagsOutput);
action!(user, tag_user, tag_user, TagUserRequest, TagUserOutput);
action!(user, untag_user, untag_user, UntagUserRequest, UntagUserOutput);
action!(user, update_user, update_user, UpdateUserRequest, UpdateUserOutput);
action!(user, put_user_policy, put_user_policy, PutUserPolicyRequest, PutUserPolicyOutput);
action!(login_profile, change_password, change_password, ChangePasswordRequest, ChangePasswordOutput);
action!(role, attach_role_policy, attach_role_policy, AttachRolePolicyRequest, AttachRolePolicyOutput);
action!(role, get_role_policy, get_role_policy, GetRolePolicyRequest, GetRolePolicyOutput);
action!(role, list_role_tags, list_role_tags, ListRoleTagsRequest, ListRoleTagsOutput);
action!(role, list_role_policies, list_role_policies, ListRolePoliciesRequest, ListRolePoliciesOutput);
action!(role, list_roles, list_roles, ListRolesRequest, ListRolesOutput);
action!(role, put_role_policy, put_role_policy, PutRolePolicyRequest, PutRolePolicyOutput);
action!(role, tag_role, tag_role, TagRoleRequest, TagRoleOutput);
action!(role, untag_role, untag_role, UntagRoleRequest, UntagRoleOutput);
action!(
    open_id_connect_provider,
    add_client_id_to_open_id_connect_provider,
    add_client_id_to_open_id_connect_provider,
    AddClientIdToOpenIdConnectProviderRequest,
    AddClientIdToOpenIdConnectProviderOutput
);
action!(
    open_id_connect_provider,
    list_open_id_connect_provider_tags,
    list_open_id_connect_provider_tags,
    ListOpenIdConnectProviderTagsRequest,
    ListOpenIdConnectProviderTagsOutput
);
action!(
    open_id_connect_provider,
    tag_open_id_connect_provider,
    tag_open_id_connect_provider,
    TagOpenIdConnectProviderRequest,
    TagOpenIdConnectProviderOutput
);
action!(
    open_id_connect_provider,
    untag_open_id_connect_provider,
    untag_open_id_connect_provider,
    UntagOpenIdConnectProviderRequest,
    UntagOpenIdConnectProviderOutput
);
action!(
    saml_provider,
    list_saml_provider_tags,
    list_saml_provider_tags,
    ListSamlProviderTagsRequest,
    ListSamlProviderTagsOutput
);
action!(saml_provider, tag_saml_provider, tag_saml_provider, TagSamlProviderRequest, TagSamlProviderOutput);
action!(saml_provider, untag_saml_provider, untag_saml_provider, UntagSamlProviderRequest, UntagSamlProviderOutput);
action!(mfa_device, get_mfa_device, get_mfa_device, GetMfaDeviceRequest, GetMfaDeviceOutput);
action!(mfa_device, enable_mfa_device, enable_mfa_device, EnableMfaDeviceRequest, EnableMfaDeviceOutput);
action!(
    mfa_device,
    list_virtual_mfa_devices,
    list_virtual_mfa_devices,
    ListVirtualMfaDevicesRequest,
    ListVirtualMfaDevicesOutput
);
action!(mfa_device, tag_mfa_device, tag_mfa_device, TagMfaDeviceRequest, TagMfaDeviceOutput);
action!(mfa_device, untag_mfa_device, untag_mfa_device, UntagMfaDeviceRequest, UntagMfaDeviceOutput);
action!(mfa_device, list_mfa_device_tags, list_mfa_device_tags, ListMfaDeviceTagsRequest, ListMfaDeviceTagsOutput);
action!(
    ssh_public_key,
    upload_ssh_public_key,
    upload_ssh_public_key,
    UploadSshPublicKeyRequest,
    UploadSshPublicKeyOutput
);
action!(
    signing_certificate,
    upload_signing_certificate,
    upload_signing_certificate,
    UploadSigningCertificateRequest,
    UploadSigningCertificateOutput
);
action!(
    server_certificate,
    upload_server_certificate,
    upload_server_certificate,
    UploadServerCertificateRequest,
    UploadServerCertificateOutput
);
action!(
    server_certificate,
    tag_server_certificate,
    tag_server_certificate,
    TagServerCertificateRequest,
    TagServerCertificateOutput
);
action!(
    server_certificate,
    untag_server_certificate,
    untag_server_certificate,
    UntagServerCertificateRequest,
    UntagServerCertificateOutput
);
action!(
    server_certificate,
    list_server_certificate_tags,
    list_server_certificate_tags,
    ListServerCertificateTagsRequest,
    ListServerCertificateTagsOutput
);
