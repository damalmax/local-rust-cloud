use aws_sdk_iam::operation::enable_mfa_device::EnableMfaDeviceOutput;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalEnableMfaDeviceOutput = OutputWrapper<EnableMfaDeviceOutput>;

impl From<LocalEnableMfaDeviceOutput> for XmlResponse {
    fn from(val: LocalEnableMfaDeviceOutput) -> Self {
        super::confirmation::xml_response("EnableMFADeviceResponse", &val.request_id)
    }
}
