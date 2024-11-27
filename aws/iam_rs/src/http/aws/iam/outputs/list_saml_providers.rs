use aws_sdk_iam::operation::list_saml_providers::ListSamlProvidersOutput;
use aws_smithy_xml::encode::XmlWriter;

use web::local::XmlResponse;
use xml::write_request_metadata_tag;

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalListSamlProvidersOutput = OutputWrapper<ListSamlProvidersOutput>;

impl From<LocalListSamlProvidersOutput> for XmlResponse {
    fn from(val: LocalListSamlProvidersOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("ListSAMLProvidersResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("ListSAMLProvidersResult").finish();
        super::saml_provider_list::write_slice(&mut result_tag, "SAMLProviderList", val.inner.saml_provider_list());
        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
