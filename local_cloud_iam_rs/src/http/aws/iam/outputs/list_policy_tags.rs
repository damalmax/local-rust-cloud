use aws_sdk_iam::operation::list_policy_tags::ListPolicyTagsOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_actix::local::web::XmlResponse;
use local_cloud_xml::write_tag_with_value;

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalListPolicyTagsOutput = OutputWrapper<ListPolicyTagsOutput>;

impl From<LocalListPolicyTagsOutput> for XmlResponse {
    fn from(val: LocalListPolicyTagsOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut list_policy_tags_response_tag = doc
            .start_el("ListPolicyTagsResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut list_policy_tags_result_tag = list_policy_tags_response_tag.start_el("ListPolicyTagsResult").finish();
        let tags = val.inner.tags();

        super::tags::write_slice(&mut list_policy_tags_result_tag, tags);

        if let Some(token) = val.inner.marker() {
            write_tag_with_value(&mut list_policy_tags_result_tag, "Marker", Some(token));
        }
        write_tag_with_value(&mut list_policy_tags_result_tag, "IsTruncated", Some(val.inner.is_truncated.to_string()));
        list_policy_tags_result_tag.finish();

        local_cloud_xml::write_request_metadata_tag(
            &mut list_policy_tags_response_tag,
            "ResponseMetadata",
            "RequestId",
            val.request_id,
        );

        list_policy_tags_response_tag.finish();
        XmlResponse(out)
    }
}
