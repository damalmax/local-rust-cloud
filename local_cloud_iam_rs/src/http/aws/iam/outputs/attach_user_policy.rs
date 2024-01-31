use aws_sdk_iam::operation::attach_user_policy::AttachUserPolicyOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalAttachUserPolicyOutput = OutputWrapper<AttachUserPolicyOutput>;

impl From<LocalAttachUserPolicyOutput> for XmlResponse {
    fn from(val: LocalAttachUserPolicyOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut attache_user_policy_response_tag = doc
            .start_el("AttachUserPolicyResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let attache_user_policy_result_tag = attache_user_policy_response_tag
            .start_el("AttachUserPolicyResult")
            .finish();
        attache_user_policy_result_tag.finish();

        local_cloud_xml::write_request_metadata_tag(
            &mut attache_user_policy_response_tag,
            "ResponseMetadata",
            "RequestId",
            val.request_id,
        );

        attache_user_policy_response_tag.finish();
        XmlResponse(out)
    }
}
