use aws_sdk_iam::operation::upload_server_certificate::UploadServerCertificateOutput;
use aws_smithy_xml::encode::XmlWriter;

use web::local::XmlResponse;
use xml::write_request_metadata_tag;

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalUploadServerCertificateOutput = OutputWrapper<UploadServerCertificateOutput>;

impl From<LocalUploadServerCertificateOutput> for XmlResponse {
    fn from(val: LocalUploadServerCertificateOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("UploadServerCertificateResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("UploadServerCertificateResult").finish();

        if let Some(metadata) = val.inner.server_certificate_metadata() {
            super::server_certificate_metadata::write(&mut result_tag, "ServerCertificateMetadata", metadata);
        }
        super::tags::write_slice(&mut result_tag, val.inner.tags());

        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
