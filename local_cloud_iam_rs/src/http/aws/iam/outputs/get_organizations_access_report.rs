use aws_sdk_iam::operation::get_organizations_access_report::GetOrganizationsAccessReportOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_actix::local::web::XmlResponse;
use local_cloud_xml::{write_iso8061_datetime_value_tag, write_request_metadata_tag, write_tag_with_value};

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalGetOrganizationsAccessReportOutput = OutputWrapper<GetOrganizationsAccessReportOutput>;

impl From<LocalGetOrganizationsAccessReportOutput> for XmlResponse {
    fn from(val: LocalGetOrganizationsAccessReportOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("GetOrganizationsAccessReportResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("GetOrganizationsAccessReportResult").finish();
        write_iso8061_datetime_value_tag(&mut result_tag, "JobCompletionDate", val.inner.job_completion_date());
        write_iso8061_datetime_value_tag(&mut result_tag, "JobCreationDate", Some(val.inner.job_creation_date()));
        write_tag_with_value(&mut result_tag, "JobStatus", Some(val.inner.job_status().as_str()));
        write_tag_with_value(
            &mut result_tag,
            "NumberOfServicesAccessible",
            val.inner.number_of_services_accessible().map(|v| v.to_string()),
        );
        write_tag_with_value(
            &mut result_tag,
            "NumberOfServicesNotAccessed",
            val.inner.number_of_services_not_accessed().map(|v| v.to_string()),
        );

        super::access_details::write_slice(&mut result_tag, "AccessDetails", val.inner.access_details());

        if let Some(token) = val.inner.marker() {
            write_tag_with_value(&mut result_tag, "Marker", Some(token));
        }
        write_tag_with_value(&mut result_tag, "IsTruncated", Some(val.inner.is_truncated.to_string()));
        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
