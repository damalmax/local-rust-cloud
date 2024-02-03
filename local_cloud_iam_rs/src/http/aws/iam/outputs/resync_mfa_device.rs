use aws_sdk_iam::operation::resync_mfa_device::ResyncMfaDeviceOutput;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalResyncMfaDeviceOutput = OutputWrapper<ResyncMfaDeviceOutput>;

impl From<LocalResyncMfaDeviceOutput> for XmlResponse {
    fn from(val: LocalResyncMfaDeviceOutput) -> Self {
        super::confirmation::xml_response("ResyncMFADeviceResponse", &val.request_id)
    }
}
