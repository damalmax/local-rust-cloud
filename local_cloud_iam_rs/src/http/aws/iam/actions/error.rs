use aws_smithy_xml::encode::XmlWriter;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::actions::tag::LocalTag;
use crate::http::aws::iam::constants;

use actix_http::StatusCode;

#[derive(Debug, Clone)]
pub enum IamErrorKind {
    InvalidInput,
    ValidationError,
    ServiceFailureException,
}

impl Into<String> for &IamErrorKind {
    fn into(self) -> String {
        match self {
            IamErrorKind::InvalidInput => String::from("InvalidInput"),
            IamErrorKind::ValidationError => String::from("ValidationError"),
            IamErrorKind::ServiceFailureException => String::from("ServiceFailureException"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum IamErrorResponseType {
    Single,
    WrappedSingle,
    Multiple,
}

#[derive(Debug, Clone)]
pub struct IamApiError {
    pub error_code: StatusCode,
    pub request_id: String,
    pub response_type: IamErrorResponseType,
    pub kind: IamErrorKind,
    pub message: String,
}

impl IamApiError {
    pub fn bad_request(request_id: &str, message: &str) -> IamApiError {
        IamApiError {
            error_code: StatusCode::BAD_REQUEST,
            request_id: request_id.to_owned(),
            response_type: IamErrorResponseType::WrappedSingle,
            kind: IamErrorKind::InvalidInput,
            message: message.to_owned(),
        }
    }

    pub fn internal_server_error(request_id: impl Into<String>, message: &str) -> IamApiError {
        IamApiError {
            error_code: StatusCode::INTERNAL_SERVER_ERROR,
            request_id: request_id.into(),
            response_type: IamErrorResponseType::WrappedSingle,
            kind: IamErrorKind::ServiceFailureException,
            message: message.to_string(),
        }
    }

    pub fn duplicate_tags(request_id: impl Into<String>) -> IamApiError {
        IamApiError {
            error_code: StatusCode::BAD_REQUEST,
            request_id: request_id.into(),
            response_type: IamErrorResponseType::Multiple,
            kind: IamErrorKind::InvalidInput,
            message: "Duplicate tag keys found. Please note that Tag keys are case insensitive.".to_string(),
        }
    }

    pub fn too_many_tags(request_id: impl Into<String>, tags: &[LocalTag], param: &str) -> IamApiError {
        IamApiError {
            error_code: StatusCode::BAD_REQUEST,
            request_id: request_id.into(),
            response_type: IamErrorResponseType::WrappedSingle,
            kind: IamErrorKind::InvalidInput,

            message: format!("1 validation error detected: Value '{:?}' at '{}' failed to satisfy constraint: Member must have length less than or equal to 50.", tags, param),
        }
    }

    pub fn tag_key_too_big(request_id: impl Into<String>, tag: &str, param: &str) -> IamApiError {
        IamApiError {
            error_code: StatusCode::BAD_REQUEST,
            request_id: request_id.into(),
            response_type: IamErrorResponseType::Multiple,
            kind: IamErrorKind::ValidationError,
            message: format!("1 validation error detected: Value '{}' at '{}' failed to satisfy constraint: Member must have length less than or equal to 128.", tag, param),
        }
    }

    pub fn tag_value_too_big(request_id: impl Into<String>, tag_value: &str, tag_index: &str) -> IamApiError {
        IamApiError {
            error_code: StatusCode::BAD_REQUEST,
            request_id: request_id.into(),
            response_type: IamErrorResponseType::Multiple,
            kind: IamErrorKind::ValidationError,
            message: format!("1 validation error detected: Value '{}' at 'Tags.member.{}.value' failed to satisfy constraint: Member must have length less than or equal to 256.", tag_value, tag_index),
        }
    }

    pub fn invalid_tag_characters(request_id: impl Into<String>, tag: &str, param: &str) -> IamApiError {
        IamApiError {
            error_code: StatusCode::BAD_REQUEST,
            request_id: request_id.into(),
            response_type: IamErrorResponseType::Multiple,
            kind: IamErrorKind::ValidationError,
            message: format!("1 validation error detected: Value '{}' at '{}' failed to satisfy constraint: Member must satisfy regular expression pattern: [\\p{{L}}\\p{{Z}}\\p{{N}}_.:/=+\\-@]+", tag, param),
        }
    }
}

impl Into<XmlResponse> for IamApiError {
    fn into(self) -> XmlResponse {
        let value = &self;
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        match self.response_type {
            IamErrorResponseType::Single => {
                let mut error_tag = doc.start_el("Error").finish();
                local_cloud_xml::write_tag_with_value(&mut error_tag, "Code", Some(&value.kind));
                local_cloud_xml::write_tag_with_value(&mut error_tag, "Message", Some(&value.message));
                local_cloud_xml::write_request_metadata_tag(
                    &mut error_tag,
                    "ResponseMetadata",
                    "RequestId",
                    &value.request_id,
                );
                error_tag.finish();
            }
            IamErrorResponseType::WrappedSingle => {
                let mut error_response_tag = doc
                    .start_el("ErrorResponse")
                    .write_ns(constants::xml::IAM_XMLNS, None)
                    .finish();
                let mut error_tag = error_response_tag.start_el("Error").finish();
                local_cloud_xml::write_tag_with_value(&mut error_tag, "Code", Some(&value.kind));
                local_cloud_xml::write_tag_with_value(&mut error_tag, "Message", Some(&value.message));
                error_tag.finish();
                local_cloud_xml::write_request_metadata_tag(
                    &mut error_response_tag,
                    "ResponseMetadata",
                    "RequestId",
                    &value.request_id,
                );
                error_response_tag.finish();
            }
            IamErrorResponseType::Multiple => {
                let mut error_response_tag = doc
                    .start_el("ErrorResponse")
                    .write_ns(constants::xml::IAM_XMLNS, None)
                    .finish();
                let mut errors_tag = error_response_tag.start_el("Errors").finish();
                let mut error_tag = errors_tag.start_el("Error").finish();
                local_cloud_xml::write_tag_with_value(&mut error_tag, "Code", Some(&value.kind));
                local_cloud_xml::write_tag_with_value(
                    &mut error_tag,
                    "Message",
                    Some(format!("<![CDATA[{}]]>", value.message)),
                );
                error_tag.finish();
                errors_tag.finish();
                local_cloud_xml::write_request_metadata_tag(
                    &mut error_response_tag,
                    "ResponseMetadata",
                    "RequestId",
                    &value.request_id,
                );
                error_response_tag.finish();
            }
        }

        return XmlResponse(out);
    }
}
