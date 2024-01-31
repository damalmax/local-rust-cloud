use aws_sdk_iam::operation::list_policies::ListPoliciesOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_actix::local::web::XmlResponse;
use local_cloud_xml::{write_iso8061_datetime_value_tag, write_tag_with_value};

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
        let mut policies_tag = list_policies_result_tag.start_el("Policies").finish();
        for policy in policies {
            let mut policy_tag = policies_tag.start_el("member").finish();
            write_tag_with_value(&mut policy_tag, "PolicyName", policy.policy_name());
            write_tag_with_value(&mut policy_tag, "PolicyId", policy.policy_id());
            write_tag_with_value(&mut policy_tag, "Arn", policy.arn());
            write_tag_with_value(&mut policy_tag, "Path", policy.path());
            write_tag_with_value(&mut policy_tag, "Description", policy.description());
            write_tag_with_value(&mut policy_tag, "DefaultVersionId", policy.default_version_id());
            write_tag_with_value(&mut policy_tag, "AttachmentCount", policy.attachment_count().map(|v| v.to_string()));
            write_tag_with_value(
                &mut policy_tag,
                "PermissionsBoundaryUsageCount",
                policy.permissions_boundary_usage_count().map(|v| v.to_string()),
            );
            write_tag_with_value(&mut policy_tag, "IsAttachable", Some(policy.is_attachable().to_string()));
            write_iso8061_datetime_value_tag(&mut policy_tag, "CreateDate", policy.create_date());
            write_iso8061_datetime_value_tag(&mut policy_tag, "UpdateDate", policy.update_date());

            super::tags::write(&mut policy_tag, policy.tags());
            policy_tag.finish();
        }
        policies_tag.finish();
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