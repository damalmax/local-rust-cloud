use aws_sdk_iam::operation::get_access_key_last_used::GetAccessKeyLastUsedOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_axum::local::web::XmlResponse;
use local_cloud_xml::{write_iso8061_datetime_value_tag, write_request_metadata_tag, write_tag_with_value};

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalGetAccessKeyLastUsedOutput = OutputWrapper<GetAccessKeyLastUsedOutput>;

impl From<LocalGetAccessKeyLastUsedOutput> for XmlResponse {
    fn from(val: LocalGetAccessKeyLastUsedOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("GetAccessKeyLastUsedResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("GetAccessKeyLastUsedResult").finish();

        write_tag_with_value(&mut result_tag, "UserName", val.inner.user_name());
        if let Some(key_last_used) = val.inner.access_key_last_used() {
            let mut key_last_used_tag = result_tag.start_el("AccessKeyLastUsed").finish();

            write_tag_with_value(&mut key_last_used_tag, "Region", Some(&key_last_used.region));
            write_iso8061_datetime_value_tag(
                &mut key_last_used_tag,
                "LastUsedDate",
                key_last_used.last_used_date(),
            );
            write_tag_with_value(&mut key_last_used_tag, "ServiceName", Some(&key_last_used.service_name));
        }
        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
