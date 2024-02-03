use aws_sdk_iam::operation::get_policy::GetPolicyOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_actix::local::web::XmlResponse;
use local_cloud_xml::write_request_metadata_tag;

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalGetPolicyOutput = OutputWrapper<GetPolicyOutput>;

impl From<LocalGetPolicyOutput> for XmlResponse {
    fn from(val: LocalGetPolicyOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("GetPolicyResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("GetPolicyResult").finish();

        if let Some(policy) = val.inner.policy() {
            super::policies::write(&mut result_tag, "Policy", policy);
        }

        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
