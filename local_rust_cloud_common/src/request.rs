use std::collections::HashMap;
use std::io::{Error, ErrorKind};

use actix_web::{http, web, HttpRequest};

const AWS_SERVICE_TARGET: &str = "X-Amz-Target";
const FORM_URLENCODED_MEDIA_TYPE: &str = "application/x-www-form-urlencoded";

#[derive(Debug)]
pub struct AwsRequest {
    pub aws_service_target: String,
    pub query_params: HashMap<String, String>,
}

impl AwsRequest {
    pub fn from_request(body_bytes: web::Bytes, req: &HttpRequest) -> Result<AwsRequest, Error> {
        if req.headers().contains_key(http::header::CONTENT_TYPE)
            && FORM_URLENCODED_MEDIA_TYPE == req.headers().get(http::header::CONTENT_TYPE).unwrap().to_str().unwrap()
        {
            let body_str = String::from_utf8(body_bytes.to_vec()).expect("failed to parse request body");
            let query = web::Query::<HashMap<String, String>>::from_query(body_str.as_str()).unwrap();

            if req.headers().contains_key(AWS_SERVICE_TARGET) {
                let aws_service_target = req
                    .headers()
                    .get(AWS_SERVICE_TARGET)
                    .unwrap() // the header is there, the `unwrap` is safe
                    .to_str()
                    .expect("failed to parse request headers");
                return Result::Ok(AwsRequest {
                    aws_service_target: String::from(aws_service_target),
                    query_params: query.0,
                });
            }
            // There is no `X-Amz-Target` header. Try to get target service name from the request body.
            let aws_service_target = query.get("Action").expect("failed to extract Action value from request").as_str();
            return Result::Ok(AwsRequest {
                aws_service_target: String::from(aws_service_target),
                query_params: query.0,
            });
        } else {
            Result::Err(Error::new(ErrorKind::InvalidInput, "unsupported request"))
        }
    }
}

#[cfg(test)]
mod tests {
    use actix_web::web::Bytes;
    use actix_web::{http, test};

    use crate::request::{AwsRequest, AWS_SERVICE_TARGET, FORM_URLENCODED_MEDIA_TYPE};

    #[test]
    async fn from_request_with_aws_service_target_header() {
        let req = test::TestRequest::default()
            .append_header((http::header::CONTENT_TYPE, FORM_URLENCODED_MEDIA_TYPE))
            .append_header((AWS_SERVICE_TARGET, "TestAction"))
            .to_http_request();

        let aws_request = AwsRequest::from_request(Bytes::from(""), &req).unwrap();

        assert_eq!(aws_request.aws_service_target, "TestAction");
    }

    #[test]
    async fn from_request_with_query_string_in_body() {
        let req = test::TestRequest::default()
            .append_header((http::header::CONTENT_TYPE, FORM_URLENCODED_MEDIA_TYPE))
            .to_http_request();
        let aws_request = AwsRequest::from_request(Bytes::from("Action=TestAction&one=1&two=2"), &req).unwrap();

        assert_eq!(aws_request.aws_service_target, "TestAction");
        assert_eq!(aws_request.query_params.get("Action").unwrap(), "TestAction");
        assert_eq!(aws_request.query_params.get("one").unwrap(), "1");
        assert_eq!(aws_request.query_params.get("two").unwrap(), "2");
    }
}
