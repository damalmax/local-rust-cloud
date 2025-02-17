use aws_sdk_iam::operation::create_group::CreateGroupOutput;
use aws_smithy_xml::encode::XmlWriter;

use web::local::XmlResponse;
use xml::write_request_metadata_tag;

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalCreateGroupOutput = OutputWrapper<CreateGroupOutput>;

impl From<LocalCreateGroupOutput> for XmlResponse {
    fn from(val: LocalCreateGroupOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("CreateGroupResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("CreateGroupResult").finish();

        if let Some(group) = val.inner.group() {
            super::groups::write(&mut result_tag, "Group", group);
        }

        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        return XmlResponse(out);
    }
}
