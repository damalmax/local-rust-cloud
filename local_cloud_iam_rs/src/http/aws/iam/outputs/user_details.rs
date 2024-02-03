use aws_sdk_iam::types::UserDetail;
use aws_smithy_xml::encode::ScopeWriter;

use local_cloud_xml::{write_iso8061_datetime_value_tag, write_tag_with_value};

pub(crate) fn write_slice(parent_tag: &mut ScopeWriter, users: &[UserDetail]) {
    let mut users_tag = parent_tag.start_el("UserDetailList").finish();
    for user in users {
        write(&mut users_tag, "member", user);
    }
    users_tag.finish()
}

pub(crate) fn write(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, user: &UserDetail) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    super::strings::write_slice(&mut wrapper_tag, "GroupList", user.group_list());

    if user.user_policy_list.is_some() {
        super::policy_details::write_slice(&mut wrapper_tag, "UserPolicyList", user.user_policy_list());
    }
    super::attached_policies::write_slice(
        &mut wrapper_tag,
        "AttachedManagedPolicies",
        user.attached_managed_policies(),
    );
    if let Some(permissions_boundary) = user.permissions_boundary() {
        super::attached_permissions_boundaries::write(&mut wrapper_tag, "PermissionsBoundary", permissions_boundary);
    }

    write_tag_with_value(&mut wrapper_tag, "Path", user.path());
    write_tag_with_value(&mut wrapper_tag, "UserName", user.user_name());
    write_tag_with_value(&mut wrapper_tag, "UserId", user.user_id());
    write_tag_with_value(&mut wrapper_tag, "Arn", user.arn());
    write_iso8061_datetime_value_tag(&mut wrapper_tag, "CreateDate", user.create_date());
    super::tags::write_slice(&mut wrapper_tag, user.tags());

    wrapper_tag.finish();
}
