use aws_sdk_iam::operation::get_server_certificate::GetServerCertificateOutput;
use aws_smithy_xml::encode::XmlWriter;

use web::local::XmlResponse;
use xml::write_request_metadata_tag;

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalGetServerCertificateOutput = OutputWrapper<GetServerCertificateOutput>;

impl From<LocalGetServerCertificateOutput> for XmlResponse {
    fn from(val: LocalGetServerCertificateOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("GetServerCertificateResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("GetServerCertificateResult").finish();
        if let Some(certificate) = val.inner.server_certificate() {
            super::server_certificates::write(&mut result_tag, "ServerCertificate", certificate);
        }
        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
