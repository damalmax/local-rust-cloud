use aws_sdk_iam::types::Role;
use aws_smithy_xml::encode::ScopeWriter;

use local_cloud_xml::{write_iso8061_datetime_value_tag, write_tag_with_value};

pub(crate) fn write_slice(parent_tag: &mut ScopeWriter, items: &[Role]) {
    let mut items_tag = parent_tag.start_el("Roles").finish();
    for item in items {
        write(&mut items_tag, "member", item);
    }
    items_tag.finish()
}

pub(crate) fn write(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, item: &Role) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    write_tag_with_value(&mut wrapper_tag, "Path", Some(item.path()));
    write_tag_with_value(&mut wrapper_tag, "RoleName", Some(item.role_name()));
    write_tag_with_value(&mut wrapper_tag, "RoleId", Some(item.role_id()));
    write_tag_with_value(&mut wrapper_tag, "Arn", Some(item.arn()));
    write_iso8061_datetime_value_tag(&mut wrapper_tag, "CreateDate", Some(item.create_date()));
    if let Some(role_last_used) = item.role_last_used() {
        let mut role_last_used_tag = wrapper_tag.start_el("RoleLastUsed").finish();
        write_iso8061_datetime_value_tag(&mut role_last_used_tag, "LastUsedDate", role_last_used.last_used_date());
        write_tag_with_value(&mut role_last_used_tag, "Region", role_last_used.region());
        role_last_used_tag.finish();
    }
    write_tag_with_value(&mut wrapper_tag, "Description", item.description());
    write_tag_with_value(&mut wrapper_tag, "MaxSessionDuration", item.max_session_duration().map(|v| v.to_string()));
    write_tag_with_value(&mut wrapper_tag, "AssumeRolePolicyDocument", item.assume_role_policy_document());
    if let Some(permissions_boundary) = item.permissions_boundary() {
        super::attached_permissions_boundaries::write(&mut wrapper_tag, "PermissionsBoundary", permissions_boundary);
    }
    super::tags::write_slice(&mut wrapper_tag, item.tags());
    wrapper_tag.finish();
}
