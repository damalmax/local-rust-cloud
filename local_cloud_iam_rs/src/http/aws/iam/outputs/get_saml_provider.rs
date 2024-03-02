use aws_sdk_iam::operation::get_saml_provider::GetSamlProviderOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_axum::local::web::XmlResponse;
use local_cloud_xml::{write_iso8061_datetime_value_tag, write_request_metadata_tag, write_tag_with_value};

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalGetSamlProviderOutput = OutputWrapper<GetSamlProviderOutput>;

impl From<LocalGetSamlProviderOutput> for XmlResponse {
    fn from(val: LocalGetSamlProviderOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("GetSAMLProviderResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("GetSAMLProviderResult").finish();

        write_tag_with_value(&mut result_tag, "SAMLMetadataDocument", val.inner.saml_metadata_document());
        write_iso8061_datetime_value_tag(&mut result_tag, "CreateDate", val.inner.create_date());
        write_iso8061_datetime_value_tag(&mut result_tag, "ValidUntil", val.inner.valid_until());
        super::tags::write_slice(&mut result_tag, val.inner.tags());

        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
