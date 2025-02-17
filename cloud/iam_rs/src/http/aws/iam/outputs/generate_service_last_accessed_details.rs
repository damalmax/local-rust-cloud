use aws_sdk_iam::operation::generate_service_last_accessed_details::GenerateServiceLastAccessedDetailsOutput;
use aws_smithy_xml::encode::XmlWriter;

use web::local::XmlResponse;
use xml::{write_request_metadata_tag, write_tag_with_value};

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalGenerateServiceLastAccessedDetailsOutput = OutputWrapper<GenerateServiceLastAccessedDetailsOutput>;

impl From<LocalGenerateServiceLastAccessedDetailsOutput> for XmlResponse {
    fn from(val: LocalGenerateServiceLastAccessedDetailsOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("GenerateServiceLastAccessedDetailsResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag
            .start_el("GenerateServiceLastAccessedDetailsResult")
            .finish();

        write_tag_with_value(&mut result_tag, "JobId", val.inner.job_id());

        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
