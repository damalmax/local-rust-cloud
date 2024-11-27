use aws_sdk_iam::operation::create_service_linked_role::CreateServiceLinkedRoleOutput;
use aws_smithy_xml::encode::XmlWriter;

use web::local::XmlResponse;
use xml::write_request_metadata_tag;

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalCreateServiceLinkedRoleOutput = OutputWrapper<CreateServiceLinkedRoleOutput>;

impl From<LocalCreateServiceLinkedRoleOutput> for XmlResponse {
    fn from(val: LocalCreateServiceLinkedRoleOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("CreateServiceLinkedRoleResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("CreateServiceLinkedRoleResult").finish();

        if let Some(role) = val.inner.role() {
            super::roles::write(&mut result_tag, "Role", role);
        }

        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
