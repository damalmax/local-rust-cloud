use aws_sdk_iam::operation::list_access_keys::ListAccessKeysOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_axum::local::web::XmlResponse;
use local_cloud_xml::{write_request_metadata_tag, write_tag_with_value};

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalListAccessKeysOutput = OutputWrapper<ListAccessKeysOutput>;

impl From<LocalListAccessKeysOutput> for XmlResponse {
    fn from(val: LocalListAccessKeysOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("ListAccessKeysResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("ListAccessKeysResult").finish();
        super::access_key_metadata::write_slice(&mut result_tag, "AccessKeyMetadata", val.inner.access_key_metadata());

        if let Some(token) = val.inner.marker() {
            write_tag_with_value(&mut result_tag, "Marker", Some(token));
        }
        write_tag_with_value(&mut result_tag, "IsTruncated", Some(val.inner.is_truncated.to_string()));
        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
