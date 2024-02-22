use actix_http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use uuid::Uuid;

use local_cloud_actix::local;
use local_cloud_actix::local::web::XmlResponse;
use local_cloud_db::LocalDb;

use crate::http::aws::iam::actions::action::Action;
use crate::http::aws::iam::actions::error::ApiError;
use crate::http::aws::iam::types::add_client_id_to_open_id_connect_provider_request::AddClientIdToOpenIdConnectProviderRequest;
use crate::http::aws::iam::types::add_role_to_instance_profile_request::AddRoleToInstanceProfileRequest;
use crate::http::aws::iam::types::add_user_to_group_request::AddUserToGroupRequest;
use crate::http::aws::iam::types::attach_group_policy_request::AttachGroupPolicyRequest;
use crate::http::aws::iam::types::attach_role_policy_request::AttachRolePolicyRequest;
use crate::http::aws::iam::types::attach_user_policy_request::AttachUserPolicyRequest;
use crate::http::aws::iam::types::change_password_request::ChangePasswordRequest;
use crate::http::aws::iam::types::create_group_request::CreateGroupRequest;
use crate::http::aws::iam::types::create_instance_profile_request::CreateInstanceProfileRequest;
use crate::http::aws::iam::types::create_login_profile_request::CreateLoginProfileRequest;
use crate::http::aws::iam::types::create_open_id_connect_provider_request::CreateOpenIdConnectProviderRequest;
use crate::http::aws::iam::types::create_policy_request::CreatePolicyRequest;
use crate::http::aws::iam::types::create_policy_version_request::CreatePolicyVersionRequest;
use crate::http::aws::iam::types::create_role_request::CreateRoleRequest;
use crate::http::aws::iam::types::create_saml_provider_request::CreateSamlProviderRequest;
use crate::http::aws::iam::types::create_user_request::CreateUserRequest;
use crate::http::aws::iam::types::create_virtual_mfa_device_request::CreateVirtualMfaDeviceRequest;
use crate::http::aws::iam::types::enable_mfa_device_request::EnableMfaDeviceRequest;
use crate::http::aws::iam::types::get_group_policy_request::GetGroupPolicyRequest;
use crate::http::aws::iam::types::get_group_request::GetGroupRequest;
use crate::http::aws::iam::types::get_mfa_device_request::GetMfaDeviceRequest;
use crate::http::aws::iam::types::get_role_policy_request::GetRolePolicyRequest;
use crate::http::aws::iam::types::get_user_policy_request::GetUserPolicyRequest;
use crate::http::aws::iam::types::list_group_policies_request::ListGroupPoliciesRequest;
use crate::http::aws::iam::types::list_groups_for_user_request::ListGroupsForUserRequest;
use crate::http::aws::iam::types::list_groups_request::ListGroupsRequest;
use crate::http::aws::iam::types::list_instance_profile_tags_request::ListInstanceProfileTagsRequest;
use crate::http::aws::iam::types::list_mfa_device_tags_request::ListMfaDeviceTagsRequest;
use crate::http::aws::iam::types::list_open_id_connect_provider_tags_request::ListOpenIdConnectProviderTagsRequest;
use crate::http::aws::iam::types::list_policies_request::ListPoliciesRequest;
use crate::http::aws::iam::types::list_policy_tags_request::ListPolicyTagsRequest;
use crate::http::aws::iam::types::list_policy_versions_request::ListPolicyVersionsRequest;
use crate::http::aws::iam::types::list_role_policies_request::ListRolePoliciesRequest;
use crate::http::aws::iam::types::list_role_tags_request::ListRoleTagsRequest;
use crate::http::aws::iam::types::list_roles_request::ListRolesRequest;
use crate::http::aws::iam::types::list_saml_provider_tags_request::ListSamlProviderTagsRequest;
use crate::http::aws::iam::types::list_server_certificate_tags_request::ListServerCertificateTagsRequest;
use crate::http::aws::iam::types::list_user_policies_request::ListUserPoliciesRequest;
use crate::http::aws::iam::types::list_user_tags_request::ListUserTagsRequest;
use crate::http::aws::iam::types::list_users_request::ListUsersRequest;
use crate::http::aws::iam::types::list_virtual_mfa_devices_request::ListVirtualMfaDevicesRequest;
use crate::http::aws::iam::types::put_group_policy_request::PutGroupPolicyRequest;
use crate::http::aws::iam::types::put_role_policy_request::PutRolePolicyRequest;
use crate::http::aws::iam::types::put_user_policy_request::PutUserPolicyRequest;
use crate::http::aws::iam::types::tag_instance_profile_request::TagInstanceProfileRequest;
use crate::http::aws::iam::types::tag_mfa_device_request::TagMfaDeviceRequest;
use crate::http::aws::iam::types::tag_open_id_connect_provider_request::TagOpenIdConnectProviderRequest;
use crate::http::aws::iam::types::tag_policy_request::TagPolicyRequest;
use crate::http::aws::iam::types::tag_role_request::TagRoleRequest;
use crate::http::aws::iam::types::tag_saml_provider_request::TagSamlProviderRequest;
use crate::http::aws::iam::types::tag_server_certificate_request::TagServerCertificateRequest;
use crate::http::aws::iam::types::tag_user_request::TagUserRequest;
use crate::http::aws::iam::types::untag_instance_profile_request::UntagInstanceProfileRequest;
use crate::http::aws::iam::types::untag_mfa_device_request::UntagMfaDeviceRequest;
use crate::http::aws::iam::types::untag_open_id_connect_provider_request::UntagOpenIdConnectProviderRequest;
use crate::http::aws::iam::types::untag_policy_request::UntagPolicyRequest;
use crate::http::aws::iam::types::untag_role_request::UntagRoleRequest;
use crate::http::aws::iam::types::untag_saml_provider_request::UntagSamlProviderRequest;
use crate::http::aws::iam::types::untag_server_certificate_request::UntagServerCertificateRequest;
use crate::http::aws::iam::types::untag_user_request::UntagUserRequest;
use crate::http::aws::iam::types::upload_server_certificate_request::UploadServerCertificateRequest;
use crate::http::aws::iam::types::upload_signing_certificate_request::UploadSigningCertificateRequest;
use crate::http::aws::iam::types::upload_ssh_public_key_request::UploadSshPublicKeyRequest;

#[derive(Deserialize, Debug)]
#[serde(tag = "Action")]
pub(crate) enum LocalAwsRequest {
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
    UploadServerCertificate(UploadServerCertificateRequest),
    UploadSigningCertificate(UploadSigningCertificateRequest),
    UploadSSHPublicKey(UploadSshPublicKeyRequest),
}

macro_rules! execute {
    ($var:ident, $account_id:ident, $aws_request_id:ident, $db:ident) => {
        $var.execute($account_id, &$aws_request_id, $db.as_ref())
            .await
            .map(|out| out.into())
    };
}

pub(crate) async fn handle(
    _req: HttpRequest, aws_query: local::web::AwsQuery<LocalAwsRequest>, db: web::Data<LocalDb>,
) -> impl Responder {
    // TODO: populate account ID from token
    let account_id = 1i64;
    let aws_request = aws_query.into_inner();
    let aws_request_id = Uuid::new_v4().to_string();
    let output: Result<XmlResponse, ApiError> = match aws_request {
        LocalAwsRequest::AddClientIDToOpenIDConnectProvider(request) => {
            execute!(request, account_id, aws_request_id, db)
        }
        LocalAwsRequest::AddRoleToInstanceProfile(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::AddUserToGroup(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::AttachGroupPolicy(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::AttachRolePolicy(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::AttachUserPolicy(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::ChangePassword(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::CreateGroup(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::CreateInstanceProfile(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::CreateOpenIDConnectProvider(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::CreateLoginProfile(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::CreatePolicy(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::CreatePolicyVersion(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::CreateRole(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::CreateSAMLProvider(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::CreateUser(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::CreateVirtualMFADevice(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::EnableMFADevice(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::GetGroup(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::GetGroupPolicy(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::GetMFADevice(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::GetRolePolicy(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::GetUserPolicy(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::ListGroups(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::ListGroupsForUser(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::ListGroupPolicies(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::ListInstanceProfileTags(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::ListMFADeviceTags(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::ListOpenIDConnectProviderTags(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::ListPolicies(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::ListPolicyVersions(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::ListPolicyTags(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::ListRoles(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::ListSAMLProviderTags(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::ListServerCertificateTags(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::ListRolePolicies(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::ListRoleTags(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::ListUserPolicies(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::ListUserTags(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::ListUsers(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::ListVirtualMFADevices(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::PutGroupPolicy(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::PutRolePolicy(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::PutUserPolicy(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::TagInstanceProfile(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::TagMFADevice(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::TagOpenIDConnectProvider(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::TagPolicy(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::TagRole(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::TagSAMLProvider(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::TagServerCertificate(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::TagUser(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::UntagInstanceProfile(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::UntagMFADevice(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::UntagOpenIDConnectProvider(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::UntagPolicy(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::UntagRole(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::UntagSAMLProvider(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::UntagServerCertificate(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::UntagUser(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::UploadServerCertificate(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::UploadSigningCertificate(request) => execute!(request, account_id, aws_request_id, db),
        LocalAwsRequest::UploadSSHPublicKey(request) => execute!(request, account_id, aws_request_id, db),
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
