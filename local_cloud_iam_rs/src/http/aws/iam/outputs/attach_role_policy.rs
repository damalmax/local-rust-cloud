use aws_sdk_iam::operation::attach_role_policy::AttachRolePolicyOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalAttachRolePolicyOutput = OutputWrapper<AttachRolePolicyOutput>;

impl From<LocalAttachRolePolicyOutput> for XmlResponse {
    fn from(val: LocalAttachRolePolicyOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut attach_role_policy_response_tag = doc
            .start_el("AttachRolePolicyResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let attache_role_policy_result_tag = attach_role_policy_response_tag
            .start_el("AttachRolePolicyResult")
            .finish();
        attache_role_policy_result_tag.finish();

        local_cloud_xml::write_request_metadata_tag(
            &mut attach_role_policy_response_tag,
            "ResponseMetadata",
            "RequestId",
            val.request_id,
        );

        attach_role_policy_response_tag.finish();
        XmlResponse(out)
    }
}
