use actix_web::{get, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthcheckResponse {
    pub status: String,
}

#[get("/healthcheck")]
async fn handle() -> impl Responder {
    let response = HealthcheckResponse { status: "Ok".to_string() };
    HttpResponse::Ok().json(response)
}
