use aws_sdk_iam::types::Group;
use aws_smithy_xml::encode::ScopeWriter;

use local_cloud_xml::{write_iso8061_datetime_value_tag, write_tag_with_value};

pub(crate) fn write_slice(parent_tag: &mut ScopeWriter, groups: &[Group]) {
    let mut groups_tag = parent_tag.start_el("Groups").finish();
    for group in groups {
        write(&mut groups_tag, "member", group);
    }
    groups_tag.finish()
}

pub(crate) fn write(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, group: &Group) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    write_tag_with_value(&mut wrapper_tag, "Path", Some(group.path()));
    write_iso8061_datetime_value_tag(&mut wrapper_tag, "CreateDate", Some(group.create_date()));
    write_tag_with_value(&mut wrapper_tag, "GroupId", Some(group.group_id()));
    write_tag_with_value(&mut wrapper_tag, "Arn", Some(group.arn()));
    write_tag_with_value(&mut wrapper_tag, "GroupName", Some(group.group_name()));
    wrapper_tag.finish();
}
