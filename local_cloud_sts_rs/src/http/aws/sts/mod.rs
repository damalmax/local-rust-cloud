use actix_http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use uuid::Uuid;

use local_cloud_actix::local;
use local_cloud_actix::local::web::XmlResponse;
use local_cloud_db::LocalDb;

use crate::http::aws::sts::actions::assume_role::LocalAssumeRole;
use crate::http::aws::sts::actions::error::StsApiError;

pub(crate) mod actions;
pub(crate) mod constants;
pub(crate) mod repository;
pub(crate) mod types;

#[derive(Deserialize, Debug)]
#[serde(tag = "Action")]
pub(crate) enum LocalAwsRequest {
    #[serde(rename = "AssumeRole")]
    AssumeRole(LocalAssumeRole),
}

pub(crate) async fn handle(
    req: HttpRequest, aws_query: local::web::AwsQuery<LocalAwsRequest>, db: web::Data<LocalDb>,
) -> impl Responder {
    // TODO: populate account ID from token
    let acc_id = 1i64;
    let aws_request = aws_query.into_inner();
    let aws_request_id = Uuid::new_v4().to_string();
    let output: Result<XmlResponse, StsApiError> = (match aws_request {
        LocalAwsRequest::AssumeRole(assume_role) => assume_role.execute(acc_id, &aws_request_id, db.as_ref()).await,
    })
    .map(|out| out.into());

    return match output {
        Ok(body) => HttpResponse::with_body(StatusCode::OK, body),
        Err(err) => {
            let error_code = err.error_code;
            let body: XmlResponse = err.into();
            HttpResponse::with_body(error_code, body)
        }
    };
}
