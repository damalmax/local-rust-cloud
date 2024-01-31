use aws_sdk_iam::operation::create_policy_version::CreatePolicyVersionOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalCreatePolicyVersionOutput = OutputWrapper<CreatePolicyVersionOutput>;

impl From<LocalCreatePolicyVersionOutput> for XmlResponse {
    fn from(val: LocalCreatePolicyVersionOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut create_policy_version_response_tag = doc
            .start_el("CreatePolicyVersionResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut create_policy_version_result_tag = create_policy_version_response_tag
            .start_el("CreatePolicyVersionResult")
            .finish();
        if val.inner.policy_version().is_some() {
            let policy_version = val.inner.policy_version().unwrap();
            let mut policy_version_tag = create_policy_version_result_tag.start_el("PolicyVersion").finish();

            local_cloud_xml::write_iso8061_datetime_value_tag(
                &mut policy_version_tag,
                "CreateDate",
                policy_version.create_date(),
            );
            local_cloud_xml::write_tag_with_value(&mut policy_version_tag, "VersionId", policy_version.version_id());
            local_cloud_xml::write_tag_with_value(
                &mut policy_version_tag,
                "IsDefaultVersion",
                Some(policy_version.is_default_version().to_string()),
            );
        }
        create_policy_version_result_tag.finish();

        local_cloud_xml::write_request_metadata_tag(
            &mut create_policy_version_response_tag,
            "ResponseMetadata",
            "RequestId",
            val.request_id,
        );

        create_policy_version_response_tag.finish();
        XmlResponse(out)
    }
}
