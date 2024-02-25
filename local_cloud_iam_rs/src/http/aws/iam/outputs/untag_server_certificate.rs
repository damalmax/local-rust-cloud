use aws_sdk_iam::operation::untag_server_certificate::UntagServerCertificateOutput;

use local_cloud_axum::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalUntagServerCertificateOutput = OutputWrapper<UntagServerCertificateOutput>;

impl From<LocalUntagServerCertificateOutput> for XmlResponse {
    fn from(val: LocalUntagServerCertificateOutput) -> Self {
        super::confirmation::xml_response("UntagServerCertificateResponse", &val.request_id)
    }
}
