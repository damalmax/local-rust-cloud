use aws_sdk_iam::{operation::create_policy::CreatePolicyOutput, types::Tag};
use aws_smithy_xml::encode::XmlWriter;

use super::{constants::IAM_XMLNS, response::IamResponse, OutputWrapper};

pub type LocalCreatePolicyOutput = OutputWrapper<CreatePolicyOutput>;

impl From<LocalCreatePolicyOutput> for IamResponse {
    fn from(val: LocalCreatePolicyOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut create_policy_response_tag = doc.start_el("CreatePolicyResponse").write_ns(IAM_XMLNS, None).finish();

        let mut create_policy_result_tag = create_policy_response_tag.start_el("CreatePolicyResult").finish();
        if val.inner.policy().is_some() {
            let policy = val.inner.policy().unwrap();
            let mut policy_tag = create_policy_result_tag.start_el("Policy").finish();
            local_rust_cloud_xml::write_tag_with_value(&mut policy_tag, "PolicyName", policy.policy_name());
            local_rust_cloud_xml::write_tag_with_value(&mut policy_tag, "PolicyId", policy.policy_id());
            local_rust_cloud_xml::write_tag_with_value(&mut policy_tag, "Arn", policy.arn());
            local_rust_cloud_xml::write_tag_with_value(&mut policy_tag, "Path", policy.path());
            local_rust_cloud_xml::write_tag_with_value(&mut policy_tag, "DefaultVersionId", policy.default_version_id());
            local_rust_cloud_xml::write_tag_with_value(
                &mut policy_tag,
                "AttachmentCount",
                policy.attachment_count().map(|v| v.to_string()),
            );
            local_rust_cloud_xml::write_tag_with_value(
                &mut policy_tag,
                "PermissionsBoundaryUsageCount",
                policy.permissions_boundary_usage_count().map(|v| v.to_string()),
            );
            local_rust_cloud_xml::write_tag_with_value(&mut policy_tag, "IsAttachable", Option::Some(policy.is_attachable().to_string()));
            local_rust_cloud_xml::write_tag_with_value(&mut policy_tag, "Description", policy.description());
            local_rust_cloud_xml::write_iso8061_datetime_value_tag(&mut policy_tag, "CreateDate", policy.create_date());
            local_rust_cloud_xml::write_iso8061_datetime_value_tag(&mut policy_tag, "UpdateDate", policy.update_date());
            local_rust_cloud_xml::write_key_value_tags(
                &mut policy_tag,
                policy.tags(),
                |t: &Tag| t.key().map(|v| v.to_string()),
                |t: &Tag| t.value().map(|v| v.to_string()),
            );
        }
        create_policy_result_tag.finish();

        local_rust_cloud_xml::write_request_metadata_tag(&mut create_policy_response_tag, "ResponseMatadata", "RequestId", val.request_id);

        create_policy_response_tag.finish();
        return out;
    }
}
