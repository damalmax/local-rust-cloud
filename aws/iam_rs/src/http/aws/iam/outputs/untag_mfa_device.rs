use aws_sdk_iam::operation::untag_mfa_device::UntagMfaDeviceOutput;

use web::local::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalUntagMfaDeviceOutput = OutputWrapper<UntagMfaDeviceOutput>;

impl From<LocalUntagMfaDeviceOutput> for XmlResponse {
    fn from(val: LocalUntagMfaDeviceOutput) -> Self {
        super::confirmation::xml_response("UntagMFADeviceResponse", &val.request_id)
    }
}
