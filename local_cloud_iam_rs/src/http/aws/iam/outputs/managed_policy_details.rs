use aws_sdk_iam::types::ManagedPolicyDetail;
use aws_smithy_xml::encode::ScopeWriter;

use local_cloud_xml::{write_iso8061_datetime_value_tag, write_tag_with_value};

pub(crate) fn write_slice(parent_tag: &mut ScopeWriter, policies: &[ManagedPolicyDetail]) {
    let mut policies_tag = parent_tag.start_el("GroupDetailList").finish();
    for policy in policies {
        write(&mut policies_tag, "member", policy);
    }
    policies_tag.finish()
}

pub(crate) fn write(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, policy: &ManagedPolicyDetail) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    write_tag_with_value(&mut wrapper_tag, "PolicyName", policy.policy_name());
    write_tag_with_value(&mut wrapper_tag, "DefaultVersionId", policy.default_version_id());
    write_tag_with_value(&mut wrapper_tag, "PolicyId", policy.policy_id());
    write_tag_with_value(&mut wrapper_tag, "Path", policy.path());
    write_tag_with_value(&mut wrapper_tag, "Arn", policy.arn());
    write_tag_with_value(&mut wrapper_tag, "AttachmentCount", policy.attachment_count().map(|v| v.to_string()));
    write_tag_with_value(&mut wrapper_tag, "IsAttachable", Some(policy.is_attachable.to_string()));
    write_iso8061_datetime_value_tag(&mut wrapper_tag, "CreateDate", policy.create_date());
    write_iso8061_datetime_value_tag(&mut wrapper_tag, "UpdateDate", policy.update_date());
    super::policy_versions::write_slice(&mut wrapper_tag, "PolicyVersionList", policy.policy_version_list());
    wrapper_tag.finish();
}
