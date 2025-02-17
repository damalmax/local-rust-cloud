use aws_sdk_iam::operation::get_user_policy::GetUserPolicyOutput;
use aws_smithy_xml::encode::XmlWriter;

use web::local::XmlResponse;
use xml::{write_request_metadata_tag, write_tag_with_value};

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalGetUserPolicyOutput = OutputWrapper<GetUserPolicyOutput>;

impl From<LocalGetUserPolicyOutput> for XmlResponse {
    fn from(val: LocalGetUserPolicyOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("GetUserPolicyResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("GetUserPolicyResult").finish();

        write_tag_with_value(&mut result_tag, "PolicyName", Some(val.inner.policy_name()));
        write_tag_with_value(&mut result_tag, "PolicyDocument", Some(val.inner.policy_document()));
        write_tag_with_value(&mut result_tag, "UserName", Some(val.inner.user_name()));

        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
