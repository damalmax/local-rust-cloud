use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

use local_cloud_common::request::AwsRequest;
use local_cloud_common::service_handler::ServiceHandler;

use crate::aws::actions::action::Iam;

pub(crate) mod types;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

pub(crate) async fn handle(body_bytes: web::Bytes, req: HttpRequest) -> impl Responder {
    let aws_request = AwsRequest::from_request(body_bytes, &req);
    return match aws_request {
        Ok(aws_request) => {
            let action_name = aws_request.aws_service_target;
            let action = Iam::from_str(&action_name);
            return action.handle(&req, aws_request.query_params);
        }
        Err(e) => {
            let response = Response { message: e.to_string() };
            HttpResponse::BadRequest().json(response)
        }
    };
}
