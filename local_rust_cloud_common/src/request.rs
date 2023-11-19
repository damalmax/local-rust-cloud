use std::collections::HashMap;
use std::fmt::Display;
use std::io::{Error, ErrorKind};

use actix_web::http::header::HeaderMap;
use actix_web::{http, web, HttpRequest};

const AWS_SERVICE_TARGET: &str = "X-Amz-Target";
const FORM_URLENCODED_MEDIA_TYPE: &str = "application/x-www-form-urlencoded";

#[derive(Debug)]
pub struct AwsRequest {
    pub aws_service_target: String,
    pub query_params: HashMap<String, String>,
}

fn is_form_urlencoded_media_type_request(headers: &HeaderMap) -> bool {
    if !headers.contains_key(http::header::CONTENT_TYPE) {
        return false;
    }
    let header_value = headers
        .get(http::header::CONTENT_TYPE)
        .map(|h| h.to_str().unwrap_or(""))
        .unwrap_or("");
    let parts: Vec<&str> = header_value.split(';').collect();
    return parts.len() != 0 && FORM_URLENCODED_MEDIA_TYPE == parts[0];
}

impl AwsRequest {
    pub fn from_request(body_bytes: web::Bytes, req: &HttpRequest) -> Result<AwsRequest, Error> {
        if is_form_urlencoded_media_type_request(req.headers()) {
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

#[derive(Debug, Clone)]
pub struct LocalTag {
    pub tag_index: String,
    pub key: String,
    pub value: String,
}

impl LocalTag {
    pub fn tag_index(&self) -> &str {
        &self.tag_index
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug)]
pub struct QueryReader {
    params: HashMap<String, String>,
}

impl QueryReader {
    pub fn new(params: HashMap<String, String>) -> Self {
        QueryReader { params }
    }

    pub fn get_string(&self, key: impl Into<String>) -> Option<String> {
        let key = key.into();
        if self.params.contains_key(key.as_str()) {
            Option::Some(self.params.get(key.as_str()).unwrap().to_string())
        } else {
            Option::None
        }
    }

    pub fn get_i32(&self, key: impl Into<String>) -> Option<i32> {
        self.get_string(key).map(|v| v.parse::<i32>().expect("Failed to parse property"))
    }

    pub fn get_i32_or_default(&self, key: impl Into<String>, default: i32) -> Option<i32> {
        self.get_i32(key).or_else(|| Option::Some(default))
    }

    fn get_tag_value(&self, key_index: &str) -> Option<String> {
        let value_tag_key = format!("Tags.member.{}.Value", key_index);
        self.params.get(&value_tag_key).map(|v| v.to_string())
    }

    pub fn get_tags(&self) -> Option<Vec<LocalTag>> {
        let mut tags: Vec<LocalTag> = vec![];
        for param in &self.params {
            let key = param.0;
            if key.starts_with("Tags.member.") && key.ends_with(".Key") {
                let parts: Vec<&str> = key.split('.').collect();
                let key_index = parts[2];
                let tag = LocalTag {
                    tag_index: key_index.to_string(),
                    key: param.1.to_string(),
                    value: self.get_tag_value(key_index).unwrap_or(String::new()),
                };
                tags.push(tag)
            }
        }
        if tags.len() > 0 {
            return Option::Some(tags);
        }
        return Option::None;
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

    #[test]
    async fn from_request_with_charset_in_content_type_and_query_string_in_body() {
        let req = test::TestRequest::default()
            .append_header((http::header::CONTENT_TYPE, FORM_URLENCODED_MEDIA_TYPE.to_string() + "; charset=utf-8"))
            .to_http_request();
        let aws_request = AwsRequest::from_request(Bytes::from("Action=TestAction&one=1&two=2"), &req).unwrap();

        assert_eq!(aws_request.aws_service_target, "TestAction");
        assert_eq!(aws_request.query_params.get("Action").unwrap(), "TestAction");
        assert_eq!(aws_request.query_params.get("one").unwrap(), "1");
        assert_eq!(aws_request.query_params.get("two").unwrap(), "2");
    }
}
