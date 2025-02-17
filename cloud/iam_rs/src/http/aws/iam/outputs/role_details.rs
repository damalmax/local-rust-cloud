use aws_sdk_iam::types::RoleDetail;
use aws_smithy_xml::encode::ScopeWriter;

use xml::{write_iso8061_datetime_value_tag, write_tag_with_value};

pub(crate) fn write_slice(parent_tag: &mut ScopeWriter, roles: &[RoleDetail]) {
    let mut roles_tag = parent_tag.start_el("RoleDetailList").finish();
    for role in roles {
        write(&mut roles_tag, "member", role);
    }
    roles_tag.finish()
}

pub(crate) fn write(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, role: &RoleDetail) {
    //12
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    write_tag_with_value(&mut wrapper_tag, "RoleName", role.role_name());
    write_tag_with_value(&mut wrapper_tag, "Arn", role.arn());
    write_tag_with_value(&mut wrapper_tag, "Path", role.path());
    write_tag_with_value(&mut wrapper_tag, "RoleId", role.role_id());
    write_tag_with_value(&mut wrapper_tag, "AssumeRolePolicyDocument", role.assume_role_policy_document());
    write_iso8061_datetime_value_tag(&mut wrapper_tag, "CreateDate", role.create_date());

    if let Some(role_last_used) = role.role_last_used() {
        let mut last_used_tag = wrapper_tag.start_el("RoleLastUsed").finish();
        write_iso8061_datetime_value_tag(&mut last_used_tag, "LastUsedDate", role_last_used.last_used_date());
        write_tag_with_value(&mut last_used_tag, "Region", role_last_used.region());
        last_used_tag.finish();
    }
    if let Some(permissions_boundary) = role.permissions_boundary() {
        super::attached_permissions_boundaries::write(&mut wrapper_tag, "PermissionsBoundary", permissions_boundary);
    }

    super::attached_policies::write_slice(
        &mut wrapper_tag,
        "AttachedManagedPolicies",
        role.attached_managed_policies(),
    );
    super::instance_profiles::write_slice(&mut wrapper_tag, "InstanceProfileList", role.instance_profile_list());
    super::tags::write_slice(&mut wrapper_tag, role.tags());
    wrapper_tag.finish();
}
