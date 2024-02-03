use aws_sdk_iam::operation::get_credential_report::GetCredentialReportOutput;
use aws_smithy_xml::encode::XmlWriter;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;

use local_cloud_actix::local::web::XmlResponse;
use local_cloud_xml::{write_iso8061_datetime_value_tag, write_request_metadata_tag, write_tag_with_value};

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalGetCredentialReportOutput = OutputWrapper<GetCredentialReportOutput>;

impl From<LocalGetCredentialReportOutput> for XmlResponse {
    fn from(val: LocalGetCredentialReportOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("GetCredentialReportResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("GetCredentialReportResult").finish();

        write_tag_with_value(&mut result_tag, "Content", val.inner.content().map(|v| STANDARD.encode(v)));
        write_tag_with_value(&mut result_tag, "ReportFormat", val.inner.report_format().map(|v| v.as_str()));
        write_iso8061_datetime_value_tag(&mut result_tag, "GeneratedTime", val.inner.generated_time());
        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
