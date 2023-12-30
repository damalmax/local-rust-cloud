use aws_smithy_xml::encode::XmlWriter;
use aws_types::request_id::RequestId;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::actions::error::IamError;
use crate::http::aws::iam::constants;

impl Into<XmlResponse> for IamError {
    fn into(self) -> XmlResponse {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);
        let mut error_response_tag = doc
            .start_el("ErrorResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();
        let mut error_tag = error_response_tag.start_el("Error").finish();
        local_cloud_xml::write_tag_with_value(&mut error_tag, "Code", Some(self.kind.as_str()));
        local_cloud_xml::write_tag_with_value(&mut error_tag, "Message", self.message);
        error_tag.finish();
        local_cloud_xml::write_request_metadata_tag(
            &mut error_response_tag,
            "ResponseMetadata",
            "RequestId",
            self.aws_request_id,
        );

        // TODO: write extras for Error Metadata
        error_response_tag.finish();
        return XmlResponse(out);
    }
}
