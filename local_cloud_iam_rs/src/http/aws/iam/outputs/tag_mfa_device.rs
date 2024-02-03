use aws_sdk_iam::operation::tag_mfa_device::TagMfaDeviceOutput;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalTagMfaDeviceOutput = OutputWrapper<TagMfaDeviceOutput>;

impl From<LocalTagMfaDeviceOutput> for XmlResponse {
    fn from(val: LocalTagMfaDeviceOutput) -> Self {
        super::confirmation::xml_response("TagMFADeviceResponse", &val.request_id)
    }
}
