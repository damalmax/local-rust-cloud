use aws_sdk_iam::operation::get_service_linked_role_deletion_status::GetServiceLinkedRoleDeletionStatusOutput;
use aws_smithy_xml::encode::XmlWriter;

use web::local::XmlResponse;
use xml::{write_request_metadata_tag, write_tag_with_value};

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalGetServiceLinkedRoleDeletionStatusOutput = OutputWrapper<GetServiceLinkedRoleDeletionStatusOutput>;

impl From<LocalGetServiceLinkedRoleDeletionStatusOutput> for XmlResponse {
    fn from(val: LocalGetServiceLinkedRoleDeletionStatusOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("GetServiceLinkedRoleDeletionStatusResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag
            .start_el("GetServiceLinkedRoleDeletionStatusResult")
            .finish();

        write_tag_with_value(&mut result_tag, "Status", Some(val.inner.status().as_str()));

        if let Some(reason_type) = val.inner.reason() {
            let mut reason_type_tag = result_tag.start_el("DeletionTaskFailureReasonType").finish();
            write_tag_with_value(&mut reason_type_tag, "Reason", reason_type.reason());
            super::role_usages::write_slice(&mut reason_type_tag, "RoleUsageList", reason_type.role_usage_list());
            reason_type_tag.finish();
        }
        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
