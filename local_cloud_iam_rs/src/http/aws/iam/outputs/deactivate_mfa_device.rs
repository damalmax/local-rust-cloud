use aws_sdk_iam::operation::deactivate_mfa_device::DeactivateMfaDeviceOutput;

use local_cloud_axum::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalDeactivateMfaDeviceOutput = OutputWrapper<DeactivateMfaDeviceOutput>;

impl From<LocalDeactivateMfaDeviceOutput> for XmlResponse {
    fn from(val: LocalDeactivateMfaDeviceOutput) -> Self {
        super::confirmation::xml_response("DeactivateMFADeviceResponse", &val.request_id)
    }
}
