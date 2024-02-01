use aws_sdk_iam::types::Role;
use aws_smithy_xml::encode::ScopeWriter;

pub(crate) fn write_slice(parent_tag: &mut ScopeWriter, roles: &[Role]) {
    let mut roles_tag = parent_tag.start_el("Roles").finish();
    for role in roles {
        write(&mut roles_tag, "member", role);
    }
    roles_tag.finish()
}

pub(crate) fn write(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, role: &Role) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    local_cloud_xml::write_tag_with_value(&mut wrapper_tag, "Path", Some(role.path()));
    local_cloud_xml::write_tag_with_value(&mut wrapper_tag, "RoleName", Some(role.role_name()));
    local_cloud_xml::write_tag_with_value(&mut wrapper_tag, "RoleId", Some(role.role_id()));
    local_cloud_xml::write_tag_with_value(&mut wrapper_tag, "Arn", Some(role.arn()));
    local_cloud_xml::write_iso8061_datetime_value_tag(&mut wrapper_tag, "CreateDate", Some(role.create_date()));
    if let Some(role_last_used) = role.role_last_used() {
        let mut role_last_used_tag = wrapper_tag.start_el("RoleLastUsed").finish();
        local_cloud_xml::write_iso8061_datetime_value_tag(
            &mut role_last_used_tag,
            "LastUsedDate",
            role_last_used.last_used_date(),
        );
        local_cloud_xml::write_tag_with_value(&mut role_last_used_tag, "Region", role_last_used.region());
        role_last_used_tag.finish();
    }
    local_cloud_xml::write_tag_with_value(
        &mut wrapper_tag,
        "AssumeRolePolicyDocument",
        role.assume_role_policy_document(),
    );
    super::tags::write_slice(&mut wrapper_tag, role.tags());
    wrapper_tag.finish();
}
