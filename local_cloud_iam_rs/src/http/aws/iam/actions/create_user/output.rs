use aws_sdk_iam::operation::create_user::CreateUserOutput;
use aws_sdk_iam::types::Tag;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::actions::wrapper::OutputWrapper;
use crate::http::aws::iam::constants;

pub type LocalCreateUserOutput = OutputWrapper<CreateUserOutput>;

impl From<LocalCreateUserOutput> for XmlResponse {
    fn from(val: LocalCreateUserOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut create_user_response_tag = doc
            .start_el("CreateUserResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut create_user_result_tag = create_user_response_tag.start_el("CreateUserResult").finish();

        if let Some(user) = val.inner.user() {
            let mut user_tag = create_user_result_tag.start_el("User").finish();
            local_cloud_xml::write_tag_with_value(&mut user_tag, "Path", Some(user.path()));
            local_cloud_xml::write_tag_with_value(&mut user_tag, "UserName", Some(user.user_name()));
            local_cloud_xml::write_tag_with_value(&mut user_tag, "UserId", Some(user.user_id()));
            local_cloud_xml::write_tag_with_value(&mut user_tag, "Arn", Some(user.arn()));
            local_cloud_xml::write_iso8061_datetime_value_tag(&mut user_tag, "CreateDate", Some(user.create_date()));
            if let Some(permissions_boundary) = user.permissions_boundary() {
                let mut permissions_boundary_tag = user_tag.start_el("PermissionsBoundary").finish();
                local_cloud_xml::write_tag_with_value(
                    &mut permissions_boundary_tag,
                    "PermissionsBoundaryType",
                    Some(permissions_boundary.permissions_boundary_type().unwrap().as_str()),
                );
                local_cloud_xml::write_tag_with_value(
                    &mut permissions_boundary_tag,
                    "PermissionsBoundaryArn",
                    permissions_boundary.permissions_boundary_arn(),
                );
                permissions_boundary_tag.finish();
            }
            local_cloud_xml::write_key_value_tags(
                &mut user_tag,
                user.tags(),
                |t: &Tag| Some(t.key().to_owned()),
                |t: &Tag| Some(t.value().to_owned()),
            );
            user_tag.finish();
        }

        create_user_result_tag.finish();

        let mut response_metadata_tag = create_user_response_tag.start_el("ResponseMetadata").finish();
        local_cloud_xml::write_tag_with_value(&mut response_metadata_tag, "RequestId", Some(val.request_id));
        response_metadata_tag.finish();

        create_user_response_tag.finish();
        return XmlResponse(out);
    }
}
