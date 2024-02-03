use aws_sdk_iam::operation::get_service_last_accessed_details::GetServiceLastAccessedDetailsOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_actix::local::web::XmlResponse;
use local_cloud_xml::{write_iso8061_datetime_value_tag, write_request_metadata_tag, write_tag_with_value};

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalGetServiceLastAccessedDetailsOutput = OutputWrapper<GetServiceLastAccessedDetailsOutput>;

impl From<LocalGetServiceLastAccessedDetailsOutput> for XmlResponse {
    fn from(val: LocalGetServiceLastAccessedDetailsOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("GetServiceLastAccessedDetailsResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("GetServiceLastAccessedDetailsResult").finish();

        write_tag_with_value(&mut result_tag, "JobStatus", Some(val.inner.job_status.as_str()));
        write_tag_with_value(&mut result_tag, "JobType", val.inner.job_type().map(|v| v.as_str()));
        write_iso8061_datetime_value_tag(&mut result_tag, "JobCreationDate", Some(val.inner.job_creation_date()));
        super::services_last_accessed::write_slice(
            &mut result_tag,
            "ServicesLastAccessed",
            val.inner.services_last_accessed(),
        );
        write_iso8061_datetime_value_tag(&mut result_tag, "JobCompletionDate", Some(val.inner.job_completion_date()));
        if let Some(token) = val.inner.marker() {
            write_tag_with_value(&mut result_tag, "Marker", Some(token));
        }
        write_tag_with_value(&mut result_tag, "IsTruncated", Some(val.inner.is_truncated.to_string()));

        if let Some(error) = val.inner.error() {
            super::error_details::write(&mut result_tag, "Error", error);
        }

        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
