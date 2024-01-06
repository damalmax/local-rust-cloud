use aws_sdk_iam::operation::list_policies::ListPoliciesOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::actions::wrapper::OutputWrapper;
use crate::http::aws::iam::constants;

pub type LocalListPoliciesOutput = OutputWrapper<ListPoliciesOutput>;

impl From<LocalListPoliciesOutput> for XmlResponse {
    fn from(val: LocalListPoliciesOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut create_policy_response_tag = doc
            .start_el("ListPoliciesResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut create_policy_result_tag = create_policy_response_tag.start_el("ListPoliciesResult").finish();

        create_policy_result_tag.finish();

        local_cloud_xml::write_request_metadata_tag(
            &mut create_policy_response_tag,
            "ResponseMetadata",
            "RequestId",
            val.request_id,
        );

        create_policy_response_tag.finish();
        return XmlResponse(out);
    }
}
