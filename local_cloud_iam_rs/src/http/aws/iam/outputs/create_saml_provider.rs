use aws_sdk_iam::operation::create_saml_provider::CreateSamlProviderOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_actix::local::web::XmlResponse;
use local_cloud_xml::{write_request_metadata_tag, write_tag_with_value};

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalCreateSamlProviderOutput = OutputWrapper<CreateSamlProviderOutput>;

impl From<LocalCreateSamlProviderOutput> for XmlResponse {
    fn from(val: LocalCreateSamlProviderOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("CreateSAMLProviderResponse ")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("CreateSAMLProviderResult").finish();

        write_tag_with_value(&mut result_tag, "SAMLProviderArn", val.inner.saml_provider_arn());
        super::tags::write_slice(&mut result_tag, val.inner.tags());

        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
