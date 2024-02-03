use aws_sdk_iam::types::PolicyVersion;
use aws_smithy_xml::encode::ScopeWriter;

use local_cloud_xml::{write_iso8061_datetime_value_tag, write_tag_with_value};

pub(crate) fn write_slice(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, policy_versions: &[PolicyVersion]) {
    let mut policy_versions_tag = parent_tag.start_el(wrapper_tag_name).finish();
    for policy_version in policy_versions {
        write(&mut policy_versions_tag, "member", policy_version);
    }
    policy_versions_tag.finish()
}

pub(crate) fn write(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, policy_version: &PolicyVersion) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    write_tag_with_value(&mut wrapper_tag, "Document", policy_version.document());
    write_tag_with_value(&mut wrapper_tag, "IsDefaultVersion", Some(policy_version.is_default_version().to_string()));
    write_tag_with_value(&mut wrapper_tag, "VersionId", policy_version.version_id());
    write_iso8061_datetime_value_tag(&mut wrapper_tag, "CreateDate", policy_version.create_date());
    wrapper_tag.finish();
}
