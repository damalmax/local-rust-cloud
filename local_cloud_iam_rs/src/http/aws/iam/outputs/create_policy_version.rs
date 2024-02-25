use aws_sdk_iam::operation::create_policy_version::CreatePolicyVersionOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_axum::local::web::XmlResponse;
use local_cloud_xml::{write_iso8061_datetime_value_tag, write_request_metadata_tag, write_tag_with_value};

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalCreatePolicyVersionOutput = OutputWrapper<CreatePolicyVersionOutput>;

impl From<LocalCreatePolicyVersionOutput> for XmlResponse {
    fn from(val: LocalCreatePolicyVersionOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("CreatePolicyVersionResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("CreatePolicyVersionResult").finish();
        if let Some(policy_version) = val.inner.policy_version() {
            let mut policy_version_tag = result_tag.start_el("PolicyVersion").finish();
            write_iso8061_datetime_value_tag(&mut policy_version_tag, "CreateDate", policy_version.create_date());
            write_tag_with_value(&mut policy_version_tag, "VersionId", policy_version.version_id());
            write_tag_with_value(
                &mut policy_version_tag,
                "IsDefaultVersion",
                Some(policy_version.is_default_version().to_string()),
            );
        }
        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
