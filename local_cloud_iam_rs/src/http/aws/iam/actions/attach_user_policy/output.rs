use aws_sdk_iam::operation::attach_user_policy::AttachUserPolicyOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::actions::wrapper::OutputWrapper;
use crate::http::aws::iam::constants;

pub type LocalAttachUserPolicyOutput = OutputWrapper<AttachUserPolicyOutput>;

impl From<LocalAttachUserPolicyOutput> for XmlResponse {
    fn from(val: LocalAttachUserPolicyOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut add_user_to_group_response_tag = doc
            .start_el("AttachUserPolicyResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut add_user_to_group_result_tag = add_user_to_group_response_tag
            .start_el("AttachUserPolicyResult")
            .finish();
        add_user_to_group_result_tag.finish();

        let mut response_metadata_tag = add_user_to_group_response_tag.start_el("ResponseMetadata").finish();
        local_cloud_xml::write_tag_with_value(&mut response_metadata_tag, "RequestId", Some(val.request_id));
        response_metadata_tag.finish();

        add_user_to_group_response_tag.finish();
        return XmlResponse(out);
    }
}
