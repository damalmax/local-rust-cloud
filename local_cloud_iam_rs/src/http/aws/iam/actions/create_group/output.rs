use aws_sdk_iam::operation::create_group::CreateGroupOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::actions::wrapper::OutputWrapper;
use crate::http::aws::iam::constants;

pub type LocalCreateGroupOutput = OutputWrapper<CreateGroupOutput>;

impl From<LocalCreateGroupOutput> for XmlResponse {
    fn from(val: LocalCreateGroupOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut create_group_response_tag = doc
            .start_el("CreateGroupResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut create_group_result_tag = create_group_response_tag.start_el("CreateGroupResult").finish();

        if let Some(group) = val.inner.group() {
            let mut group_tag = create_group_result_tag.start_el("Group").finish();
            local_cloud_xml::write_tag_with_value(&mut group_tag, "Path", Some(group.path()));
            local_cloud_xml::write_iso8061_datetime_value_tag(&mut group_tag, "CreateDate", Some(group.create_date()));
            local_cloud_xml::write_tag_with_value(&mut group_tag, "GroupId", Some(group.group_id()));
            local_cloud_xml::write_tag_with_value(&mut group_tag, "Arn", Some(group.arn()));
            local_cloud_xml::write_tag_with_value(&mut group_tag, "GroupName", Some(group.group_name()));
            group_tag.finish();
        }

        create_group_result_tag.finish();

        let mut response_metadata_tag = create_group_response_tag.start_el("ResponseMetadata").finish();
        local_cloud_xml::write_tag_with_value(&mut response_metadata_tag, "RequestId", Some(val.request_id));
        response_metadata_tag.finish();

        create_group_response_tag.finish();
        return XmlResponse(out);
    }
}
