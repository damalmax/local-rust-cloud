use aws_sdk_iam::operation::attach_role_policy::AttachRolePolicyOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::actions::wrapper::OutputWrapper;
use crate::http::aws::iam::constants;

pub type LocalAttachRolePolicyOutput = OutputWrapper<AttachRolePolicyOutput>;

impl From<LocalAttachRolePolicyOutput> for XmlResponse {
    fn from(val: LocalAttachRolePolicyOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut add_user_to_group_response_tag = doc
            .start_el("AttachRolePolicyResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut add_user_to_group_result_tag = add_user_to_group_response_tag
            .start_el("AttachRolePolicyResult")
            .finish();
        add_user_to_group_result_tag.finish();

        let mut response_metadata_tag = add_user_to_group_response_tag.start_el("ResponseMetadata").finish();
        local_cloud_xml::write_tag_with_value(&mut response_metadata_tag, "RequestId", Some(val.request_id));
        response_metadata_tag.finish();

        add_user_to_group_response_tag.finish();
        return XmlResponse(out);
    }
}
