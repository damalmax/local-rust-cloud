use aws_smithy_xml::encode::XmlWriter;

use web::local::XmlResponse;
use xml::write_request_metadata_tag;

use crate::http::aws::iam::constants;

pub(crate) fn xml_response(response_tag_name: &str, request_id: &str) -> XmlResponse {
    let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    let mut doc = XmlWriter::new(&mut out);

    let mut response_tag = doc
        .start_el(response_tag_name)
        .write_ns(constants::xml::IAM_XMLNS, None)
        .finish();

    write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", request_id);

    response_tag.finish();
    XmlResponse(out)
}
