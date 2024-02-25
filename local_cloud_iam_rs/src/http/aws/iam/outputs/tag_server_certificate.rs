use aws_sdk_iam::operation::tag_server_certificate::TagServerCertificateOutput;

use local_cloud_axum::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalTagServerCertificateOutput = OutputWrapper<TagServerCertificateOutput>;

impl From<LocalTagServerCertificateOutput> for XmlResponse {
    fn from(val: LocalTagServerCertificateOutput) -> Self {
        super::confirmation::xml_response("TagServerCertificateResponse", &val.request_id)
    }
}
