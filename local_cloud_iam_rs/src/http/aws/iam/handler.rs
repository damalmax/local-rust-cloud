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
use crate::http::aws::iam::types::get_group_policy_request::GetGroupPolicyRequest;
use crate::http::aws::iam::types::get_group_request::GetGroupRequest;
use crate::http::aws::iam::types::get_role_policy_request::GetRolePolicyRequest;
use crate::http::aws::iam::types::get_user_policy_request::GetUserPolicyRequest;
use crate::http::aws::iam::types::list_group_policies_request::ListGroupPoliciesRequest;
use crate::http::aws::iam::types::list_groups_for_user_request::ListGroupsForUserRequest;
use crate::http::aws::iam::types::list_groups_request::ListGroupsRequest;
use crate::http::aws::iam::types::list_instance_profile_tags_request::ListInstanceProfileTagsRequest;
use crate::http::aws::iam::types::list_open_id_connect_provider_tags_request::ListOpenIdConnectProviderTagsRequest;
use crate::http::aws::iam::types::list_policies_request::ListPoliciesRequest;
use crate::http::aws::iam::types::list_policy_tags_request::ListPolicyTagsRequest;
use crate::http::aws::iam::types::list_policy_versions_request::ListPolicyVersionsRequest;
use crate::http::aws::iam::types::list_role_policies_request::ListRolePoliciesRequest;
use crate::http::aws::iam::types::list_role_tags_request::ListRoleTagsRequest;
use crate::http::aws::iam::types::list_roles_request::ListRolesRequest;
use crate::http::aws::iam::types::list_user_policies_request::ListUserPoliciesRequest;
use crate::http::aws::iam::types::list_user_tags_request::ListUserTagsRequest;
use crate::http::aws::iam::types::list_users_request::ListUsersRequest;
use crate::http::aws::iam::types::put_group_policy_request::PutGroupPolicyRequest;
use crate::http::aws::iam::types::put_role_policy_request::PutRolePolicyRequest;
use crate::http::aws::iam::types::put_user_policy_request::PutUserPolicyRequest;
use crate::http::aws::iam::types::tag_instance_profile_request::TagInstanceProfileRequest;
use crate::http::aws::iam::types::tag_policy_request::TagPolicyRequest;
use crate::http::aws::iam::types::tag_role_request::TagRoleRequest;
use crate::http::aws::iam::types::tag_user_request::TagUserRequest;

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
    GetGroup(GetGroupRequest),
    GetGroupPolicy(GetGroupPolicyRequest),
    GetRolePolicy(GetRolePolicyRequest),
    GetUserPolicy(GetUserPolicyRequest),
    ListGroups(ListGroupsRequest),
    ListGroupsForUser(ListGroupsForUserRequest),
    ListGroupPolicies(ListGroupPoliciesRequest),
    ListInstanceProfileTags(ListInstanceProfileTagsRequest),
    ListOpenIDConnectProviderTags(ListOpenIdConnectProviderTagsRequest),
    ListPolicies(ListPoliciesRequest),
    ListPolicyVersions(ListPolicyVersionsRequest),
    ListPolicyTags(ListPolicyTagsRequest),
    ListRoles(ListRolesRequest),
    ListRolePolicies(ListRolePoliciesRequest),
    ListRoleTags(ListRoleTagsRequest),
    ListUserPolicies(ListUserPoliciesRequest),
    ListUserTags(ListUserTagsRequest),
    ListUsers(ListUsersRequest),
    PutGroupPolicy(PutGroupPolicyRequest),
    PutRolePolicy(PutRolePolicyRequest),
    PutUserPolicy(PutUserPolicyRequest),
    TagInstanceProfile(TagInstanceProfileRequest),
    TagPolicy(TagPolicyRequest),
    TagRole(TagRoleRequest),
    TagUser(TagUserRequest),
}

pub(crate) async fn handle(
    _req: HttpRequest, aws_query: local::web::AwsQuery<LocalAwsRequest>, db: web::Data<LocalDb>,
) -> impl Responder {
    // TODO: populate account ID from token
    let account_id = 1i64;
    let aws_request = aws_query.into_inner();
    let aws_request_id = Uuid::new_v4().to_string();
    let output: Result<XmlResponse, ApiError> = match aws_request {
        LocalAwsRequest::AddClientIDToOpenIDConnectProvider(add_client_id_request) => add_client_id_request
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::AddRoleToInstanceProfile(add_role_to_instance_profile) => add_role_to_instance_profile
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::AddUserToGroup(add_user_to_group) => add_user_to_group
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::AttachGroupPolicy(attach_group_policy) => attach_group_policy
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::AttachRolePolicy(attach_role_policy) => attach_role_policy
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::AttachUserPolicy(attach_user_policy) => attach_user_policy
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::ChangePassword(change_password) => change_password
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::CreateGroup(create_group) => create_group
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::CreateInstanceProfile(create_instance_profile) => create_instance_profile
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::CreateOpenIDConnectProvider(create_open_id_provider) => create_open_id_provider
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::CreateLoginProfile(create_login_profile) => create_login_profile
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::CreatePolicy(create_policy) => create_policy
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::CreatePolicyVersion(create_policy_version) => create_policy_version
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::CreateRole(create_role) => create_role
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::CreateSAMLProvider(create_saml_provider) => create_saml_provider
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::CreateUser(create_user) => create_user
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::GetGroup(get_group) => get_group
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::GetGroupPolicy(get_group_policy) => get_group_policy
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::GetRolePolicy(get_role_policy) => get_role_policy
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::GetUserPolicy(get_user_policy) => get_user_policy
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::ListGroups(list_groups) => list_groups
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::ListGroupsForUser(list_groups_for_user) => list_groups_for_user
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::ListGroupPolicies(list_group_policies) => list_group_policies
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::ListInstanceProfileTags(list_instance_profile_tags) => list_instance_profile_tags
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::ListOpenIDConnectProviderTags(provider_tags) => provider_tags
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::ListPolicies(list_policies) => list_policies
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::ListPolicyVersions(list_policy_versions) => list_policy_versions
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::ListPolicyTags(list_policy_tags) => list_policy_tags
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::ListRoles(list_roles) => list_roles
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::ListRolePolicies(list_role_policies) => list_role_policies
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::ListRoleTags(list_role_tags) => list_role_tags
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::ListUserPolicies(list_user_policies) => list_user_policies
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::ListUserTags(list_user_tags) => list_user_tags
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::ListUsers(list_users) => list_users
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::PutGroupPolicy(put_group_policy) => put_group_policy
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::PutRolePolicy(put_role_policy) => put_role_policy
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::PutUserPolicy(put_user_policy) => put_user_policy
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::TagInstanceProfile(tag_instance_profile) => tag_instance_profile
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::TagPolicy(tag_policy) => tag_policy
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::TagRole(tag_role) => tag_role
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::TagUser(tag_user) => tag_user
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
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
