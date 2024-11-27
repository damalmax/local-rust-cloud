use aws_sdk_iam::operation::list_roles::ListRolesOutput;
use aws_smithy_xml::encode::XmlWriter;

use web::local::XmlResponse;
use xml::{write_request_metadata_tag, write_tag_with_value};

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalListRolesOutput = OutputWrapper<ListRolesOutput>;

impl From<LocalListRolesOutput> for XmlResponse {
    fn from(val: LocalListRolesOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("ListRolesResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("ListRolesResult").finish();
        let roles = val.inner.roles();

        super::roles::write_slice(&mut result_tag, roles);

        if let Some(token) = val.inner.marker() {
            write_tag_with_value(&mut result_tag, "Marker", Some(token));
        }
        write_tag_with_value(&mut result_tag, "IsTruncated", Some(val.inner.is_truncated.to_string()));
        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
