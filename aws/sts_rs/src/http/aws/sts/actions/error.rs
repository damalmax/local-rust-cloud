use aws_smithy_xml::encode::XmlWriter;
use axum::http::StatusCode;

use web::local::XmlResponse;

use crate::http::aws::sts::constants;

#[derive(Debug, Clone)]
pub enum StsErrorKind {
    InvalidInput,
    ServiceFailureException,
}

impl StsErrorKind {
    pub(crate) fn status_code(&self) -> StatusCode {
        match self {
            StsErrorKind::InvalidInput => StatusCode::BAD_REQUEST,
            StsErrorKind::ServiceFailureException => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl Into<String> for &StsErrorKind {
    fn into(self) -> String {
        match self {
            StsErrorKind::InvalidInput => String::from("InvalidInput"),
            StsErrorKind::ServiceFailureException => String::from("ServiceFailureException"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StsApiError {
    pub error_code: StatusCode,
    pub kind: StsErrorKind,
    pub request_id: String,
    pub message: String,
}

impl Into<XmlResponse> for StsApiError {
    fn into(self) -> XmlResponse {
        let value = &self;
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut error_response_tag = doc
            .start_el("ErrorResponse")
            .write_ns(constants::xml::STS_XMLNS, None)
            .finish();
        let mut error_tag = error_response_tag.start_el("Error").finish();
        xml::write_tag_with_value(&mut error_tag, "Code", Some(&value.kind));
        xml::write_tag_with_value(&mut error_tag, "Message", Some(&value.message));
        error_tag.finish();
        xml::write_request_metadata_tag(
            &mut error_response_tag,
            "ResponseMetadata",
            "RequestId",
            &value.request_id,
        );
        error_response_tag.finish();

        XmlResponse(out)
    }
}
