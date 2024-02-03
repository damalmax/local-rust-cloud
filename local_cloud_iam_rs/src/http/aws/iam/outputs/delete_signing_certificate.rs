use aws_sdk_iam::operation::delete_signing_certificate::DeleteSigningCertificateOutput;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalDeleteSigningCertificateOutput = OutputWrapper<DeleteSigningCertificateOutput>;

impl From<LocalDeleteSigningCertificateOutput> for XmlResponse {
    fn from(val: LocalDeleteSigningCertificateOutput) -> Self {
        super::confirmation::xml_response("DeleteSigningCertificateResponse", &val.request_id)
    }
}
