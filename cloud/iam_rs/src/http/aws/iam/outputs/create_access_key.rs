use aws_sdk_iam::operation::create_access_key::CreateAccessKeyOutput;
use aws_smithy_xml::encode::XmlWriter;

use web::local::XmlResponse;
use xml::write_request_metadata_tag;

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalCreateAccessKeyOutput = OutputWrapper<CreateAccessKeyOutput>;

impl From<LocalCreateAccessKeyOutput> for XmlResponse {
    fn from(val: LocalCreateAccessKeyOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("CreateAccessKeyResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("CreateAccessKeyResult").finish();

        if let Some(access_key) = val.inner.access_key() {
            super::access_keys::write(&mut result_tag, "AccessKey", access_key);
        }

        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
