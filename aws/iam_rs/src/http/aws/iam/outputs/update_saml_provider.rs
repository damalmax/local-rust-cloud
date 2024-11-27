use aws_sdk_iam::operation::update_saml_provider::UpdateSamlProviderOutput;
use aws_smithy_xml::encode::XmlWriter;

use web::local::XmlResponse;
use xml::{write_request_metadata_tag, write_tag_with_value};

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalUpdateSamlProviderOutput = OutputWrapper<UpdateSamlProviderOutput>;

impl From<LocalUpdateSamlProviderOutput> for XmlResponse {
    fn from(val: LocalUpdateSamlProviderOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("UpdateSAMLProviderResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("UpdateSAMLProviderResult").finish();

        write_tag_with_value(&mut result_tag, "SAMLProviderArn", val.inner.saml_provider_arn());

        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
