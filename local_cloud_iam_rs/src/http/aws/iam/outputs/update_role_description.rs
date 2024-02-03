use aws_sdk_iam::operation::update_role_description::UpdateRoleDescriptionOutput;
use aws_smithy_xml::encode::XmlWriter;

use crate::http::aws::iam::constants;
use local_cloud_actix::local::web::XmlResponse;
use local_cloud_xml::write_request_metadata_tag;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalUpdateRoleDescriptionOutput = OutputWrapper<UpdateRoleDescriptionOutput>;

impl From<LocalUpdateRoleDescriptionOutput> for XmlResponse {
    fn from(val: LocalUpdateRoleDescriptionOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("UpdateRoleDescriptionResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("UpdateRoleDescriptionResult").finish();

        if let Some(role) = val.inner.role() {
            super::roles::write(&mut result_tag, "Role", role);
        }

        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
