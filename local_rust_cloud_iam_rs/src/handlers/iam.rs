use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use local_rust_cloud_common::{request::AwsRequest, service_handler::ServiceHandler};
use serde::Serialize;

use crate::aws::handlers::action::Iam;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[post("/iam/")]
async fn handle(body_bytes: web::Bytes, req: HttpRequest) -> impl Responder {
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
