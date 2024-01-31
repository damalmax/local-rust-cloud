use aws_sdk_iam::operation::create_role::CreateRoleOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalCreateRoleOutput = OutputWrapper<CreateRoleOutput>;

impl From<LocalCreateRoleOutput> for XmlResponse {
    fn from(val: LocalCreateRoleOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut create_role_response_tag = doc
            .start_el("CreateRoleResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut create_role_result_tag = create_role_response_tag.start_el("CreateRoleResult").finish();

        if let Some(role) = val.inner.role() {
            super::roles::write(&mut create_role_result_tag, "Role", role);
        }

        create_role_result_tag.finish();

        local_cloud_xml::write_request_metadata_tag(
            &mut create_role_response_tag,
            "ResponseMetadata",
            "RequestId",
            val.request_id,
        );

        create_role_response_tag.finish();
        XmlResponse(out)
    }
}
