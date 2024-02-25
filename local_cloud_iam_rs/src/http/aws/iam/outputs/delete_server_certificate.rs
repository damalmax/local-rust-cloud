use aws_sdk_iam::operation::delete_server_certificate::DeleteServerCertificateOutput;

use local_cloud_axum::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalDeleteServerCertificateOutput = OutputWrapper<DeleteServerCertificateOutput>;

impl From<LocalDeleteServerCertificateOutput> for XmlResponse {
    fn from(val: LocalDeleteServerCertificateOutput) -> Self {
        super::confirmation::xml_response("DeleteServerCertificateResponse", &val.request_id)
    }
}
