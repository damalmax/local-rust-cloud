use aws_sdk_iam::operation::get_group_policy::GetGroupPolicyOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_axum::local::web::XmlResponse;
use local_cloud_xml::{write_request_metadata_tag, write_tag_with_value};

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalGetGroupPolicyOutput = OutputWrapper<GetGroupPolicyOutput>;

impl From<LocalGetGroupPolicyOutput> for XmlResponse {
    fn from(val: LocalGetGroupPolicyOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("GetGroupPolicyResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("GetGroupPolicyResult").finish();
        write_tag_with_value(&mut result_tag, "GroupName", Some(val.inner.group_name()));
        write_tag_with_value(&mut result_tag, "PolicyName", Some(val.inner.policy_name()));
        write_tag_with_value(&mut result_tag, "PolicyDocument", Some(val.inner.policy_document()));
        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
