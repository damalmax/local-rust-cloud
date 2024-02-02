use aws_sdk_iam::operation::create_policy::CreatePolicyOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_actix::local::web::XmlResponse;
use local_cloud_xml::write_request_metadata_tag;

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalCreatePolicyOutput = OutputWrapper<CreatePolicyOutput>;

impl From<LocalCreatePolicyOutput> for XmlResponse {
    fn from(val: LocalCreatePolicyOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("CreatePolicyResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("CreatePolicyResult").finish();
        if let Some(policy) = val.inner.policy() {
            super::policies::write(&mut result_tag, "Policy", policy);
        }
        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        return XmlResponse(out);
    }
}
