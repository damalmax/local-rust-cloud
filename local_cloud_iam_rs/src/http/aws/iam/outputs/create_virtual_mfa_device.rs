use aws_sdk_iam::operation::create_virtual_mfa_device::CreateVirtualMfaDeviceOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_actix::local::web::XmlResponse;
use local_cloud_xml::write_request_metadata_tag;

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalCreateVirtualMfaDeviceOutput = OutputWrapper<CreateVirtualMfaDeviceOutput>;

impl From<LocalCreateVirtualMfaDeviceOutput> for XmlResponse {
    fn from(val: LocalCreateVirtualMfaDeviceOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("CreateVirtualMFADeviceResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("CreateVirtualMFADeviceResult").finish();

        if let Some(device) = val.inner.virtual_mfa_device() {
            super::virtual_mfa_devices::write(&mut result_tag, "VirtualMFADevice", device);
        }

        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
