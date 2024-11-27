use aws_sdk_iam::types::GroupDetail;
use aws_smithy_xml::encode::ScopeWriter;

use xml::{write_iso8061_datetime_value_tag, write_tag_with_value};

pub(crate) fn write_slice(parent_tag: &mut ScopeWriter, groups: &[GroupDetail]) {
    let mut groups_tag = parent_tag.start_el("GroupDetailList").finish();
    for group in groups {
        write(&mut groups_tag, "member", group);
    }
    groups_tag.finish()
}

pub(crate) fn write(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, group: &GroupDetail) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    write_tag_with_value(&mut wrapper_tag, "GroupId", group.group_id());
    super::attached_policies::write_slice(
        &mut wrapper_tag,
        "AttachedManagedPolicies",
        group.attached_managed_policies(),
    );
    write_tag_with_value(&mut wrapper_tag, "GroupName", group.group_name());
    write_tag_with_value(&mut wrapper_tag, "Path", group.path());
    write_tag_with_value(&mut wrapper_tag, "Arn", group.arn());
    write_iso8061_datetime_value_tag(&mut wrapper_tag, "CreateDate", group.create_date());
    super::policy_details::write_slice(&mut wrapper_tag, "GroupPolicyList", group.group_policy_list());
    wrapper_tag.finish();
}
