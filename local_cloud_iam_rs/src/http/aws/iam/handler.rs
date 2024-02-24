use actix_http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use uuid::Uuid;

use local_cloud_actix::local;
use local_cloud_actix::local::web::XmlResponse;
use local_cloud_db::LocalDb;

use crate::http::aws::iam::actions::action::Action;
use crate::http::aws::iam::actions::error::ApiError;
use crate::http::aws::iam::types::add_client_id_to_open_id_connect_provider::AddClientIdToOpenIdConnectProviderRequest;
use crate::http::aws::iam::types::add_role_to_instance_profile::AddRoleToInstanceProfileRequest;
use crate::http::aws::iam::types::add_user_to_group::AddUserToGroupRequest;
use crate::http::aws::iam::types::attach_group_policy::AttachGroupPolicyRequest;
use crate::http::aws::iam::types::attach_role_policy::AttachRolePolicyRequest;
use crate::http::aws::iam::types::attach_user_policy::AttachUserPolicyRequest;
use crate::http::aws::iam::types::change_password::ChangePasswordRequest;
use crate::http::aws::iam::types::create_group::CreateGroupRequest;
use crate::http::aws::iam::types::create_instance_profile::CreateInstanceProfileRequest;
use crate::http::aws::iam::types::create_login_profile::CreateLoginProfileRequest;
use crate::http::aws::iam::types::create_open_id_connect_provider::CreateOpenIdConnectProviderRequest;
use crate::http::aws::iam::types::create_policy::CreatePolicyRequest;
use crate::http::aws::iam::types::create_policy_version::CreatePolicyVersionRequest;
use crate::http::aws::iam::types::create_role::CreateRoleRequest;
use crate::http::aws::iam::types::create_saml_provider::CreateSamlProviderRequest;
use crate::http::aws::iam::types::create_user::CreateUserRequest;
use crate::http::aws::iam::types::create_virtual_mfa_device::CreateVirtualMfaDeviceRequest;
use crate::http::aws::iam::types::enable_mfa_device::EnableMfaDeviceRequest;
use crate::http::aws::iam::types::get_group::GetGroupRequest;
use crate::http::aws::iam::types::get_group_policy::GetGroupPolicyRequest;
use crate::http::aws::iam::types::get_mfa_device::GetMfaDeviceRequest;
use crate::http::aws::iam::types::get_role_policy::GetRolePolicyRequest;
use crate::http::aws::iam::types::get_user_policy::GetUserPolicyRequest;
use crate::http::aws::iam::types::list_group_policies::ListGroupPoliciesRequest;
use crate::http::aws::iam::types::list_groups::ListGroupsRequest;
use crate::http::aws::iam::types::list_groups_for_user::ListGroupsForUserRequest;
use crate::http::aws::iam::types::list_instance_profile_tags::ListInstanceProfileTagsRequest;
use crate::http::aws::iam::types::list_mfa_device_tags::ListMfaDeviceTagsRequest;
use crate::http::aws::iam::types::list_open_id_connect_provider_tags::ListOpenIdConnectProviderTagsRequest;
use crate::http::aws::iam::types::list_policies::ListPoliciesRequest;
use crate::http::aws::iam::types::list_policy_tags::ListPolicyTagsRequest;
use crate::http::aws::iam::types::list_policy_versions::ListPolicyVersionsRequest;
use crate::http::aws::iam::types::list_role_policies::ListRolePoliciesRequest;
use crate::http::aws::iam::types::list_role_tags::ListRoleTagsRequest;
use crate::http::aws::iam::types::list_roles::ListRolesRequest;
use crate::http::aws::iam::types::list_saml_provider_tags::ListSamlProviderTagsRequest;
use crate::http::aws::iam::types::list_server_certificate_tags::ListServerCertificateTagsRequest;
use crate::http::aws::iam::types::list_user_policies::ListUserPoliciesRequest;
use crate::http::aws::iam::types::list_user_tags::ListUserTagsRequest;
use crate::http::aws::iam::types::list_users::ListUsersRequest;
use crate::http::aws::iam::types::list_virtual_mfa_devices::ListVirtualMfaDevicesRequest;
use crate::http::aws::iam::types::put_group_policy::PutGroupPolicyRequest;
use crate::http::aws::iam::types::put_role_policy::PutRolePolicyRequest;
use crate::http::aws::iam::types::put_user_policy::PutUserPolicyRequest;
use crate::http::aws::iam::types::tag_instance_profile::TagInstanceProfileRequest;
use crate::http::aws::iam::types::tag_mfa_device::TagMfaDeviceRequest;
use crate::http::aws::iam::types::tag_open_id_connect_provider::TagOpenIdConnectProviderRequest;
use crate::http::aws::iam::types::tag_policy::TagPolicyRequest;
use crate::http::aws::iam::types::tag_role::TagRoleRequest;
use crate::http::aws::iam::types::tag_saml_provider::TagSamlProviderRequest;
use crate::http::aws::iam::types::tag_server_certificate::TagServerCertificateRequest;
use crate::http::aws::iam::types::tag_user::TagUserRequest;
use crate::http::aws::iam::types::untag_instance_profile::UntagInstanceProfileRequest;
use crate::http::aws::iam::types::untag_mfa_device::UntagMfaDeviceRequest;
use crate::http::aws::iam::types::untag_open_id_connect_provider::UntagOpenIdConnectProviderRequest;
use crate::http::aws::iam::types::untag_policy::UntagPolicyRequest;
use crate::http::aws::iam::types::untag_role::UntagRoleRequest;
use crate::http::aws::iam::types::untag_saml_provider::UntagSamlProviderRequest;
use crate::http::aws::iam::types::untag_server_certificate::UntagServerCertificateRequest;
use crate::http::aws::iam::types::untag_user::UntagUserRequest;
use crate::http::aws::iam::types::update_user::UpdateUserRequest;
use crate::http::aws::iam::types::upload_server_certificate::UploadServerCertificateRequest;
use crate::http::aws::iam::types::upload_signing_certificate::UploadSigningCertificateRequest;
use crate::http::aws::iam::types::upload_ssh_public_key::UploadSshPublicKeyRequest;

macro_rules! action_handler {
    (
        $(#[$m:meta])*
        pub(crate) enum $name:ident {
            $($variant:ident($request:ident)),+
            $(,)?
        }
    ) => {
        $(#[$m])*
        #[non_exhaustive]
        pub(crate) enum $name {
            $(
                $variant($request)
            ),+
        }

        pub(crate) async fn handle(_req: HttpRequest,
                                   aws_query: local::web::AwsQuery<AwsRequest>,
                                   db: web::Data<LocalDb>,
                                  ) -> impl Responder {
            let account_id = 1i64;
            let aws_request = aws_query.into_inner();
            let aws_request_id = Uuid::new_v4().to_string();
            let output: Result<XmlResponse, ApiError> = match aws_request {
                $(
                    $name::$variant(request) => request.execute(account_id, &aws_request_id, db.as_ref())
                        .await
                        .map(|out| out.into()),
                )+
            };
            match output {
                Ok(body) => HttpResponse::with_body(StatusCode::OK, body),
                Err(err) => {
                    let error_code = err.kind.status_code();
                    let body: XmlResponse = err.into();
                    HttpResponse::with_body(error_code, body)
                }
            }
        }
    };
}

action_handler! {
    #[derive(serde::Deserialize, Debug)]
    #[serde(tag = "Action")]
    pub(crate) enum AwsRequest {
        AddClientIDToOpenIDConnectProvider(AddClientIdToOpenIdConnectProviderRequest),
        AddRoleToInstanceProfile(AddRoleToInstanceProfileRequest),
        AddUserToGroup(AddUserToGroupRequest),
        AttachGroupPolicy(AttachGroupPolicyRequest),
        AttachRolePolicy(AttachRolePolicyRequest),
        AttachUserPolicy(AttachUserPolicyRequest),
        ChangePassword(ChangePasswordRequest),
        CreateGroup(CreateGroupRequest),
        CreateInstanceProfile(CreateInstanceProfileRequest),
        CreateLoginProfile(CreateLoginProfileRequest),
        CreateOpenIDConnectProvider(CreateOpenIdConnectProviderRequest),
        CreatePolicy(CreatePolicyRequest),
        CreatePolicyVersion(CreatePolicyVersionRequest),
        CreateRole(CreateRoleRequest),
        CreateSAMLProvider(CreateSamlProviderRequest),
        CreateUser(CreateUserRequest),
        CreateVirtualMFADevice(CreateVirtualMfaDeviceRequest),
        EnableMFADevice(EnableMfaDeviceRequest),
        GetGroup(GetGroupRequest),
        GetGroupPolicy(GetGroupPolicyRequest),
        GetMFADevice(GetMfaDeviceRequest),
        GetRolePolicy(GetRolePolicyRequest),
        GetUserPolicy(GetUserPolicyRequest),
        ListGroups(ListGroupsRequest),
        ListGroupsForUser(ListGroupsForUserRequest),
        ListGroupPolicies(ListGroupPoliciesRequest),
        ListInstanceProfileTags(ListInstanceProfileTagsRequest),
        ListMFADeviceTags(ListMfaDeviceTagsRequest),
        ListOpenIDConnectProviderTags(ListOpenIdConnectProviderTagsRequest),
        ListPolicies(ListPoliciesRequest),
        ListPolicyVersions(ListPolicyVersionsRequest),
        ListPolicyTags(ListPolicyTagsRequest),
        ListRoles(ListRolesRequest),
        ListSAMLProviderTags(ListSamlProviderTagsRequest),
        ListServerCertificateTags(ListServerCertificateTagsRequest),
        ListRolePolicies(ListRolePoliciesRequest),
        ListRoleTags(ListRoleTagsRequest),
        ListUserPolicies(ListUserPoliciesRequest),
        ListUserTags(ListUserTagsRequest),
        ListUsers(ListUsersRequest),
        ListVirtualMFADevices(ListVirtualMfaDevicesRequest),
        PutGroupPolicy(PutGroupPolicyRequest),
        PutRolePolicy(PutRolePolicyRequest),
        PutUserPolicy(PutUserPolicyRequest),
        TagInstanceProfile(TagInstanceProfileRequest),
        TagMFADevice(TagMfaDeviceRequest),
        TagOpenIDConnectProvider(TagOpenIdConnectProviderRequest),
        TagPolicy(TagPolicyRequest),
        TagRole(TagRoleRequest),
        TagSAMLProvider(TagSamlProviderRequest),
        TagServerCertificate(TagServerCertificateRequest),
        TagUser(TagUserRequest),
        UntagInstanceProfile(UntagInstanceProfileRequest),
        UntagMFADevice(UntagMfaDeviceRequest),
        UntagOpenIDConnectProvider(UntagOpenIdConnectProviderRequest),
        UntagPolicy(UntagPolicyRequest),
        UntagRole(UntagRoleRequest),
        UntagSAMLProvider(UntagSamlProviderRequest),
        UntagServerCertificate(UntagServerCertificateRequest),
        UntagUser(UntagUserRequest),
        UpdateUser(UpdateUserRequest),
        UploadServerCertificate(UploadServerCertificateRequest),
        UploadSigningCertificate(UploadSigningCertificateRequest),
        UploadSSHPublicKey(UploadSshPublicKeyRequest),
    }
}
