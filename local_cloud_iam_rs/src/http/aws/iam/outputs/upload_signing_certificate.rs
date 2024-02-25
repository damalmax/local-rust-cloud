use aws_sdk_iam::operation::upload_signing_certificate::UploadSigningCertificateOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_axum::local::web::XmlResponse;
use local_cloud_xml::write_request_metadata_tag;

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalUploadSigningCertificateOutput = OutputWrapper<UploadSigningCertificateOutput>;

impl From<LocalUploadSigningCertificateOutput> for XmlResponse {
    fn from(val: LocalUploadSigningCertificateOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("UploadSigningCertificateResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("UploadSigningCertificateResult").finish();

        if let Some(certificate) = val.inner.certificate() {
            super::signing_certificates::write(&mut result_tag, "Certificate", certificate);
        }
        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
