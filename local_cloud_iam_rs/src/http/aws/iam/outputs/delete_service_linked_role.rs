use aws_sdk_iam::operation::delete_service_linked_role::DeleteServiceLinkedRoleOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_axum::local::web::XmlResponse;
use local_cloud_xml::{write_request_metadata_tag, write_tag_with_value};

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalDeleteServiceLinkedRoleOutput = OutputWrapper<DeleteServiceLinkedRoleOutput>;

impl From<LocalDeleteServiceLinkedRoleOutput> for XmlResponse {
    fn from(val: LocalDeleteServiceLinkedRoleOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("DeleteServiceLinkedRoleResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("DeleteServiceLinkedRoleResult").finish();

        write_tag_with_value(&mut result_tag, "DeletionTaskId", Some(val.inner.deletion_task_id()));

        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
