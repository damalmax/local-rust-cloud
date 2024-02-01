use actix_http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use uuid::Uuid;

use local_cloud_actix::local;
use local_cloud_actix::local::web::XmlResponse;
use local_cloud_db::LocalDb;

use crate::http::aws::iam::actions::action::Action;
use crate::http::aws::iam::actions::error::ApiError;
use crate::http::aws::iam::types::add_role_to_instance_profile_request::AddRoleToInstanceProfileRequest;
use crate::http::aws::iam::types::add_user_to_group_request::AddUserToGroupRequest;
use crate::http::aws::iam::types::attach_group_policy_request::AttachGroupPolicyRequest;
use crate::http::aws::iam::types::attach_role_policy_request::AttachRolePolicyRequest;
use crate::http::aws::iam::types::attach_user_policy_request::AttachUserPolicyRequest;
use crate::http::aws::iam::types::change_password_request::ChangePasswordRequest;
use crate::http::aws::iam::types::create_group_request::CreateGroupRequest;
use crate::http::aws::iam::types::create_instance_profile_request::CreateInstanceProfileRequest;
use crate::http::aws::iam::types::create_login_profile_request::CreateLoginProfileRequest;
use crate::http::aws::iam::types::create_policy_request::CreatePolicyRequest;
use crate::http::aws::iam::types::create_policy_version_request::CreatePolicyVersionRequest;
use crate::http::aws::iam::types::create_role_request::CreateRoleRequest;
use crate::http::aws::iam::types::create_user_request::CreateUserRequest;
use crate::http::aws::iam::types::get_group_request::GetGroupRequest;
use crate::http::aws::iam::types::list_groups_request::ListGroupsRequest;
use crate::http::aws::iam::types::list_policies_request::ListPoliciesRequest;
use crate::http::aws::iam::types::list_policy_tags_request::ListPolicyTagsRequest;
use crate::http::aws::iam::types::list_users_request::ListUsersRequest;

#[derive(Deserialize, Debug)]
#[serde(tag = "Action")]
pub(crate) enum LocalAwsRequest {
    #[serde(rename = "AddRoleToInstanceProfile")]
    AddRoleToInstanceProfile(AddRoleToInstanceProfileRequest),
    #[serde(rename = "AddUserToGroup")]
    AddUserToGroup(AddUserToGroupRequest),
    #[serde(rename = "AttachGroupPolicy")]
    AttachGroupPolicy(AttachGroupPolicyRequest),
    #[serde(rename = "AttachRolePolicy")]
    AttachRolePolicy(AttachRolePolicyRequest),
    #[serde(rename = "AttachUserPolicy")]
    AttachUserPolicy(AttachUserPolicyRequest),
    #[serde(rename = "ChangePassword")]
    ChangePassword(ChangePasswordRequest),
    #[serde(rename = "CreateGroup")]
    CreateGroup(CreateGroupRequest),
    #[serde(rename = "CreateInstanceProfile")]
    CreateInstanceProfile(CreateInstanceProfileRequest),
    #[serde(rename = "CreateLoginProfile")]
    CreateLoginProfile(CreateLoginProfileRequest),
    #[serde(rename = "CreatePolicy")]
    CreatePolicy(CreatePolicyRequest),
    #[serde(rename = "CreatePolicyVersion")]
    CreatePolicyVersion(CreatePolicyVersionRequest),
    #[serde(rename = "CreateRole")]
    CreateRole(CreateRoleRequest),
    #[serde(rename = "CreateUser")]
    CreateUser(CreateUserRequest),
    #[serde(rename = "GetGroup")]
    GetGroupRequest(GetGroupRequest),
    #[serde(rename = "ListGroups")]
    ListGroups(ListGroupsRequest),
    #[serde(rename = "ListPolicies")]
    ListPolicies(ListPoliciesRequest),
    #[serde(rename = "ListPolicyTags")]
    ListPolicyTags(ListPolicyTagsRequest),
    #[serde(rename = "ListUsers")]
    ListUsers(ListUsersRequest),
}

pub(crate) async fn handle(
    _req: HttpRequest, aws_query: local::web::AwsQuery<LocalAwsRequest>, db: web::Data<LocalDb>,
) -> impl Responder {
    // TODO: populate account ID from token
    let account_id = 1i64;
    let aws_request = aws_query.into_inner();
    let aws_request_id = Uuid::new_v4().to_string();
    let output: Result<XmlResponse, ApiError> = match aws_request {
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
        LocalAwsRequest::CreateUser(create_user) => create_user
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::GetGroupRequest(get_group) => get_group
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::ListGroups(list_groups) => list_groups
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::ListPolicies(list_policies) => list_policies
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::ListPolicyTags(list_policy_tags) => list_policy_tags
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::ListUsers(list_users) => list_users
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
