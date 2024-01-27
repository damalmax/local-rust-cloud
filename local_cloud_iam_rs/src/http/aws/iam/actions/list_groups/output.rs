use aws_sdk_iam::operation::list_groups::ListGroupsOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_actix::local::web::XmlResponse;
use local_cloud_xml::{write_iso8061_datetime_value_tag, write_tag_with_value};

use crate::http::aws::iam::actions::wrapper::OutputWrapper;
use crate::http::aws::iam::constants;

pub type LocalListGroupsOutput = OutputWrapper<ListGroupsOutput>;

impl From<LocalListGroupsOutput> for XmlResponse {
    fn from(val: LocalListGroupsOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut list_groups_response_tag = doc
            .start_el("ListGroupsResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut list_groups_result_tag = list_groups_response_tag.start_el("ListGroupsResult").finish();
        let groups = val.inner.groups();
        let mut groups_tag = list_groups_result_tag.start_el("Groups").finish();
        for group in groups {
            let mut group_tag = groups_tag.start_el("member").finish();
            write_tag_with_value(&mut group_tag, "Path", Some(group.path()));
            write_iso8061_datetime_value_tag(&mut group_tag, "CreateDate", Some(group.create_date()));
            write_tag_with_value(&mut group_tag, "GroupId", Some(group.group_id()));
            write_tag_with_value(&mut group_tag, "Arn", Some(group.arn()));
            write_tag_with_value(&mut group_tag, "GroupName", Some(group.group_name()));
            group_tag.finish();
        }
        groups_tag.finish();
        if let Some(token) = val.inner.marker() {
            write_tag_with_value(&mut list_groups_result_tag, "Marker", Some(token));
        }
        write_tag_with_value(&mut list_groups_result_tag, "IsTruncated", Some(val.inner.is_truncated.to_string()));
        list_groups_result_tag.finish();

        local_cloud_xml::write_request_metadata_tag(
            &mut list_groups_response_tag,
            "ResponseMetadata",
            "RequestId",
            val.request_id,
        );

        list_groups_response_tag.finish();
        return XmlResponse(out);
    }
}
