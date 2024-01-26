use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use uuid::Uuid;

use local_cloud_actix::local;
use local_cloud_actix::local::web::XmlResponse;
use local_cloud_db::LocalDb;

use crate::http::aws::iam::actions::error::ApiError;
use crate::http::aws::iam::types::create_policy_request::CreatePolicyRequest;
use crate::http::aws::iam::types::create_policy_version_request::CreatePolicyVersionRequest;
use crate::http::aws::iam::types::create_role_request::CreateRoleRequest;
use crate::http::aws::iam::types::create_user_request::CreateUserRequest;
use crate::http::aws::iam::types::list_policies_request::ListPoliciesRequest;

pub(crate) mod actions;
pub(crate) mod constants;
pub(crate) mod db;
pub(crate) mod operations;
pub(crate) mod types;
pub(crate) mod utils;
pub(crate) mod validate;

#[derive(Deserialize, Debug)]
#[serde(tag = "Action")]
pub(crate) enum LocalAwsRequest {
    #[serde(rename = "CreatePolicy")]
    CreatePolicy(CreatePolicyRequest),
    #[serde(rename = "ListPolicies")]
    ListPolicies(ListPoliciesRequest),
    #[serde(rename = "CreatePolicyVersion")]
    CreatePolicyVersion(CreatePolicyVersionRequest),
    #[serde(rename = "CreateUser")]
    CreateUser(CreateUserRequest),
    #[serde(rename = "CreateRole")]
    CreateRole(CreateRoleRequest),
}

pub(crate) async fn handle(
    _req: HttpRequest, aws_query: local::web::AwsQuery<LocalAwsRequest>, db: web::Data<LocalDb>,
) -> impl Responder {
    // TODO: populate account ID from token
    let account_id = 1i64;
    let aws_request = aws_query.into_inner();
    let aws_request_id = Uuid::new_v4().to_string();
    let output: Result<XmlResponse, ApiError> = match aws_request {
        LocalAwsRequest::CreatePolicy(create_policy) => create_policy
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::ListPolicies(list_policies) => list_policies
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::CreatePolicyVersion(create_policy_version) => create_policy_version
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::CreateUser(create_user) => create_user
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
        LocalAwsRequest::CreateRole(create_role) => create_role
            .execute(account_id, &aws_request_id, db.as_ref())
            .await
            .map(|out| out.into()),
    };

    return match output {
        Ok(body) => HttpResponse::with_body(StatusCode::OK, body),
        Err(err) => {
            let error_code = err.kind.status_code();
            let body: XmlResponse = err.into();
            HttpResponse::with_body(error_code, body)
        }
    };
}
