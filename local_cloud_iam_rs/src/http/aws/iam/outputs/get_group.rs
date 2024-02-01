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
        super::user::write_slice(&mut get_group_result_tag, val.inner.users());
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
