use aws_sdk_iam::operation::get_group::GetGroupOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_actix::local::web::XmlResponse;
use local_cloud_xml::{write_iso8061_datetime_value_tag, write_tag_with_value};

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalGetGroupOutput = OutputWrapper<GetGroupOutput>;

impl From<LocalGetGroupOutput> for XmlResponse {
    fn from(val: LocalGetGroupOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut list_groups_response_tag = doc
            .start_el("GetGroupResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut get_group_result_tag = list_groups_response_tag.start_el("GetGroupResult").finish();
        if let Some(group) = val.inner.group() {
            let mut group_tag = get_group_result_tag.start_el("Group").finish();
            write_tag_with_value(&mut group_tag, "Path", Some(group.path()));
            write_iso8061_datetime_value_tag(&mut group_tag, "CreateDate", Some(group.create_date()));
            write_tag_with_value(&mut group_tag, "GroupId", Some(group.group_id()));
            write_tag_with_value(&mut group_tag, "Arn", Some(group.arn()));
            write_tag_with_value(&mut group_tag, "GroupName", Some(group.group_name()));
            group_tag.finish();
        }

        let mut users_tag = get_group_result_tag.start_el("Users").finish();
        for user in val.inner.users() {
            let mut user_tag = users_tag.start_el("member").finish();
            write_tag_with_value(&mut user_tag, "Path", Some(user.path()));
            write_tag_with_value(&mut user_tag, "UserName", Some(user.user_name()));
            write_tag_with_value(&mut user_tag, "UserId", Some(user.user_id()));
            write_tag_with_value(&mut user_tag, "Arn", Some(user.arn()));
            write_iso8061_datetime_value_tag(&mut user_tag, "CreateDate", Some(user.create_date()));
            if let Some(permissions_boundary) = user.permissions_boundary() {
                let mut permissions_boundary_tag = user_tag.start_el("PermissionsBoundary").finish();
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
            write_iso8061_datetime_value_tag(&mut user_tag, "PasswordLastUsed", user.password_last_used());
            super::tags::write(&mut user_tag, user.tags());
            user_tag.finish();
        }
        users_tag.finish();

        if let Some(token) = val.inner.marker() {
            write_tag_with_value(&mut get_group_result_tag, "Marker", Some(token));
        }
        write_tag_with_value(&mut get_group_result_tag, "IsTruncated", Some(val.inner.is_truncated.to_string()));
        get_group_result_tag.finish();

        local_cloud_xml::write_request_metadata_tag(
            &mut list_groups_response_tag,
            "ResponseMetadata",
            "RequestId",
            val.request_id,
        );

        list_groups_response_tag.finish();
        XmlResponse(out)
    }
}
