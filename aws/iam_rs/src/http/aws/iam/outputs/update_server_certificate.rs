use aws_sdk_iam::operation::update_server_certificate::UpdateServerCertificateOutput;

use web::local::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalUpdateServerCertificateOutput = OutputWrapper<UpdateServerCertificateOutput>;

impl From<LocalUpdateServerCertificateOutput> for XmlResponse {
    fn from(val: LocalUpdateServerCertificateOutput) -> Self {
        super::confirmation::xml_response("UpdateServerCertificateResponse", &val.request_id)
    }
}
