use aws_sdk_iam::operation::delete_virtual_mfa_device::DeleteVirtualMfaDeviceOutput;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalDeleteVirtualMfaDeviceOutput = OutputWrapper<DeleteVirtualMfaDeviceOutput>;

impl From<LocalDeleteVirtualMfaDeviceOutput> for XmlResponse {
    fn from(val: LocalDeleteVirtualMfaDeviceOutput) -> Self {
        super::confirmation::xml_response("DeleteVirtualMFADeviceResponse", &val.request_id)
    }
}
