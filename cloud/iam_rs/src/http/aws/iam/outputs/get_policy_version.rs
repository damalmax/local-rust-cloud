use aws_sdk_iam::operation::get_policy_version::GetPolicyVersionOutput;
use aws_smithy_xml::encode::XmlWriter;

use web::local::XmlResponse;
use xml::write_request_metadata_tag;

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalGetPolicyVersionOutput = OutputWrapper<GetPolicyVersionOutput>;

impl From<LocalGetPolicyVersionOutput> for XmlResponse {
    fn from(val: LocalGetPolicyVersionOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("GetPolicyVersionResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("GetPolicyVersionResult").finish();

        if let Some(policy_version) = val.inner.policy_version() {
            super::policy_versions::write(&mut result_tag, "PolicyVersion", policy_version);
        }

        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
