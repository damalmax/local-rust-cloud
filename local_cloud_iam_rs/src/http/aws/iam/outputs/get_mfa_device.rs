use aws_sdk_iam::operation::get_mfa_device::GetMfaDeviceOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_axum::local::web::XmlResponse;
use local_cloud_xml::{write_iso8061_datetime_value_tag, write_request_metadata_tag, write_tag_with_value};

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalGetMfaDeviceOutput = OutputWrapper<GetMfaDeviceOutput>;

impl From<LocalGetMfaDeviceOutput> for XmlResponse {
    fn from(val: LocalGetMfaDeviceOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("GetMFADeviceResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("GetMFADeviceResult").finish();
        write_iso8061_datetime_value_tag(&mut result_tag, "EnableDate", val.inner.enable_date());
        write_tag_with_value(&mut result_tag, "SerialNumber", Some(val.inner.serial_number()));
        write_tag_with_value(&mut result_tag, "UserName", val.inner.user_name());

        if let Some(certifications) = val.inner.certifications() {
            let mut certifications_tag = result_tag.start_el("Certifications").finish();
            for (key, value) in certifications {
                let mut entry_tag = certifications_tag.start_el("entry").finish();
                write_tag_with_value(&mut entry_tag, "key", Some(key));
                write_tag_with_value(&mut entry_tag, "value", Some(value));
                entry_tag.finish();
            }
            certifications_tag.finish();
        }

        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
