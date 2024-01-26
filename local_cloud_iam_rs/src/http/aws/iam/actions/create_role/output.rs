use aws_sdk_iam::operation::create_role::CreateRoleOutput;
use aws_sdk_iam::types::Tag;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::actions::wrapper::OutputWrapper;
use crate::http::aws::iam::constants;

pub type LocalCreateRoleOutput = OutputWrapper<CreateRoleOutput>;

impl From<LocalCreateRoleOutput> for XmlResponse {
    fn from(val: LocalCreateRoleOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut create_role_response_tag = doc
            .start_el("CreateRoleResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut create_role_result_tag = create_role_response_tag.start_el("CreateRoleResult").finish();

        if let Some(role) = val.inner.role() {
            let mut role_tag = create_role_result_tag.start_el("Role").finish();
            local_cloud_xml::write_tag_with_value(&mut role_tag, "Path", Some(role.path()));
            local_cloud_xml::write_tag_with_value(&mut role_tag, "RoleName", Some(role.role_name()));
            local_cloud_xml::write_tag_with_value(&mut role_tag, "RoleId", Some(role.role_id()));
            local_cloud_xml::write_tag_with_value(&mut role_tag, "Arn", Some(role.arn()));
            local_cloud_xml::write_iso8061_datetime_value_tag(&mut role_tag, "CreateDate", Some(role.create_date()));
            local_cloud_xml::write_tag_with_value(
                &mut role_tag,
                "AssumeRolePolicyDocument",
                role.assume_role_policy_document(),
            );
            local_cloud_xml::write_key_value_tags(
                &mut role_tag,
                role.tags(),
                |t: &Tag| Some(t.key().to_owned()),
                |t: &Tag| Some(t.value().to_owned()),
            );
            role_tag.finish();
        }

        create_role_result_tag.finish();

        let mut response_metadata_tag = create_role_response_tag.start_el("ResponseMetadata").finish();
        local_cloud_xml::write_tag_with_value(&mut response_metadata_tag, "RequestId", Some(val.request_id));
        response_metadata_tag.finish();

        create_role_response_tag.finish();
        return XmlResponse(out);
    }
}
