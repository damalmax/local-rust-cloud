use axum::extract::State;
use axum::http::{Response, StatusCode};
use serde::Deserialize;
use uuid::Uuid;

use local_cloud_axum::local::web::{AwsQueryBody, XmlResponse};
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

const CONTENT_TYPE_HEADER: &str = "Content-Type";
const CONTENT_TYPE_HEADER_VALUE: &str = "text/xml; charset=utf-8";

pub(crate) async fn handle(State(db): State<LocalDb>, aws_query: AwsQueryBody<LocalAwsRequest>) -> Response<String> {
    // TODO: populate account ID from token
    let acc_id = 1i64;
    let aws_request = aws_query.into_inner();
    let aws_request_id = Uuid::new_v4().to_string();
    let output: Result<XmlResponse, StsApiError> = (match aws_request {
        LocalAwsRequest::AssumeRole(assume_role) => assume_role.execute(acc_id, &aws_request_id, &db).await,
    })
    .map(|out| out.into());

    match output {
        Ok(body) => Response::builder()
            .header(CONTENT_TYPE_HEADER, CONTENT_TYPE_HEADER_VALUE)
            .status(StatusCode::OK)
            .body(body.to_owned())
            .unwrap(),
        Err(err) => {
            let error_code = err.kind.status_code();
            let body: XmlResponse = err.into();
            Response::builder()
                .header(CONTENT_TYPE_HEADER, CONTENT_TYPE_HEADER_VALUE)
                .status(error_code)
                .body(body.to_owned())
                .unwrap()
        }
    }
}
