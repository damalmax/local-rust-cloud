use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;

use local_cloud_actix::local;
use local_cloud_actix::local::web::XmlResponse;
use local_cloud_common::service_handler::ServiceHandler;
use local_cloud_db::Database;

use crate::http::aws::iam::actions::create_policy::LocalCreatePolicy;
use crate::http::aws::iam::actions::create_user::LocalCreateUser;
use crate::http::aws::iam::actions::error::IamError;

pub(crate) mod actions;
pub(crate) mod constants;
pub(crate) mod repository;
mod types;

#[derive(Deserialize, Debug)]
#[serde(tag = "Action")]
pub(crate) enum LocalAwsRequest {
    #[serde(rename = "CreatePolicy")]
    CreatePolicy(LocalCreatePolicy),
    #[serde(rename = "CreateUser")]
    CreateUser(LocalCreateUser),
}

pub(crate) async fn handle(
    req: HttpRequest, aws_query: local::web::AwsQuery<LocalAwsRequest>, db: web::Data<Database>,
) -> impl Responder {
    // TODO: populate account ID from token
    let acc_id = 1i64;
    let aws_request = aws_query.into_inner();
    let output: Result<XmlResponse, IamError> = match aws_request {
        LocalAwsRequest::CreatePolicy(create_policy) => {
            create_policy.execute(acc_id, db.as_ref()).map(|out| out.into())
        }
        LocalAwsRequest::CreateUser(create_user) => create_user.execute(acc_id, db.as_ref()).map(|out| out.into()),
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
