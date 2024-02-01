use aws_sdk_iam::operation::list_policies::ListPoliciesOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_actix::local::web::XmlResponse;
use local_cloud_xml::write_tag_with_value;

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalListPoliciesOutput = OutputWrapper<ListPoliciesOutput>;

impl From<LocalListPoliciesOutput> for XmlResponse {
    fn from(val: LocalListPoliciesOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut list_policies_response_tag = doc
            .start_el("ListPoliciesResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut list_policies_result_tag = list_policies_response_tag.start_el("ListPoliciesResult").finish();
        let policies = val.inner.policies();

        super::policy::write_slice(&mut list_policies_result_tag, policies);
        if let Some(token) = val.inner.marker() {
            write_tag_with_value(&mut list_policies_result_tag, "Marker", Some(token));
        }
        write_tag_with_value(&mut list_policies_result_tag, "IsTruncated", Some(val.inner.is_truncated.to_string()));
        list_policies_result_tag.finish();

        local_cloud_xml::write_request_metadata_tag(
            &mut list_policies_response_tag,
            "ResponseMetadata",
            "RequestId",
            val.request_id,
        );

        list_policies_response_tag.finish();
        XmlResponse(out)
    }
}
