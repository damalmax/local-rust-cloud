use aws_sdk_iam::operation::get_context_keys_for_custom_policy::GetContextKeysForCustomPolicyOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_axum::local::web::XmlResponse;
use local_cloud_xml::{write_request_metadata_tag, write_tag_with_value};

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalGetContextKeysForCustomPolicyOutput = OutputWrapper<GetContextKeysForCustomPolicyOutput>;

impl From<LocalGetContextKeysForCustomPolicyOutput> for XmlResponse {
    fn from(val: LocalGetContextKeysForCustomPolicyOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("GetContextKeysForCustomPolicyResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("GetContextKeysForCustomPolicyResult").finish();

        let mut names_tag = result_tag.start_el("ContextKeyNames").finish();
        for key_name in val.inner.context_key_names() {
            write_tag_with_value(&mut names_tag, "member", Some(key_name));
        }
        names_tag.finish();

        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
