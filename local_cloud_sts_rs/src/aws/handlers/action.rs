use std::collections::HashMap;

use actix_web::body::BoxBody;
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::{HttpRequest, HttpResponse};
use futures::executor::block_on;
use uuid::Uuid;

use local_cloud_common::service_handler::ServiceHandler;
use local_cloud_db::Database;

use crate::aws::handlers::query::QueryReader;

#[derive(std::fmt::Debug)]
pub enum Sts {
    AssumeRole,
    AssumeRoleWithWebIdentity,
    GetSessionToken,
    GetFederationToken,
    Unknown,
}

impl Sts {
    pub fn from_str(action_name: &str) -> Self {
        match action_name {
            "AssumeRole" => Sts::AssumeRole,
            "AssumeRoleWithWebIdentity" => Sts::AssumeRoleWithWebIdentity,
            "GetSessionToken" => Sts::GetSessionToken,
            "GetFederationToken" => Sts::GetFederationToken,
            _ => Sts::Unknown,
        }
    }
}

impl ServiceHandler for Sts {
    fn handle(self, req: &HttpRequest, params: HashMap<String, String>) -> HttpResponse {
        let request_id = Uuid::new_v4().to_string();

        let query_reader = QueryReader::new(params);
        let db = req
            .app_data::<Data<Database>>()
            .expect("failed to get access to DB")
            .as_ref();
        let mut response = match self {
            Sts::AssumeRole => {
                let output = block_on(Sts::assume_role(db, request_id, query_reader)).unwrap();
                let body: String = output.into();
                HttpResponse::with_body(StatusCode::OK, BoxBody::new(body))
            }
            Sts::AssumeRoleWithWebIdentity => HttpResponse::with_body(StatusCode::OK, BoxBody::new("")),
            Sts::GetSessionToken => HttpResponse::with_body(StatusCode::OK, BoxBody::new("")),
            Sts::GetFederationToken => HttpResponse::with_body(StatusCode::OK, BoxBody::new("")),
            _ => HttpResponse::with_body(StatusCode::BAD_REQUEST, BoxBody::new("")),
        };
        response
            .headers_mut()
            .append(HeaderName::from_static("content-type"), HeaderValue::from_static("application/xml"));
        return response;
    }
}
