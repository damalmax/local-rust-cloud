use aws_sdk_iam::types::User;
use aws_smithy_xml::encode::ScopeWriter;

use local_cloud_xml::{write_iso8061_datetime_value_tag, write_tag_with_value};

pub(crate) fn write_slice(parent_tag: &mut ScopeWriter, users: &[User]) {
    let mut users_tag = parent_tag.start_el("Users").finish();
    for user in users {
        write(&mut users_tag, "member", user);
    }
    users_tag.finish()
}

pub(crate) fn write(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, user: &User) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    write_tag_with_value(&mut wrapper_tag, "Path", Some(user.path()));
    write_tag_with_value(&mut wrapper_tag, "UserName", Some(user.user_name()));
    write_tag_with_value(&mut wrapper_tag, "UserId", Some(user.user_id()));
    write_tag_with_value(&mut wrapper_tag, "Arn", Some(user.arn()));
    write_iso8061_datetime_value_tag(&mut wrapper_tag, "CreateDate", Some(user.create_date()));
    if let Some(permissions_boundary) = user.permissions_boundary() {
        let mut permissions_boundary_tag = wrapper_tag.start_el("PermissionsBoundary").finish();
        write_tag_with_value(
            &mut permissions_boundary_tag,
            "PermissionsBoundaryType",
            Some(permissions_boundary.permissions_boundary_type().unwrap().as_str()),
        );
        write_tag_with_value(
            &mut permissions_boundary_tag,
            "PermissionsBoundaryArn",
            permissions_boundary.permissions_boundary_arn(),
        );
        permissions_boundary_tag.finish();
    }
    if user.password_last_used().is_some() {
        write_iso8061_datetime_value_tag(&mut wrapper_tag, "PasswordLastUsed", user.password_last_used());
    }
    super::tags::write(&mut wrapper_tag, user.tags());
    wrapper_tag.finish();
}
