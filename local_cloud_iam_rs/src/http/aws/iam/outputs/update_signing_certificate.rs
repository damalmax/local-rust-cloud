use aws_sdk_iam::operation::update_signing_certificate::UpdateSigningCertificateOutput;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalUpdateSigningCertificateOutput = OutputWrapper<UpdateSigningCertificateOutput>;

impl From<LocalUpdateSigningCertificateOutput> for XmlResponse {
    fn from(val: LocalUpdateSigningCertificateOutput) -> Self {
        super::confirmation::xml_response("UpdateSigningCertificateResponse", &val.request_id)
    }
}
