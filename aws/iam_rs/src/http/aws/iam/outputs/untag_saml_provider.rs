use aws_sdk_iam::operation::untag_saml_provider::UntagSamlProviderOutput;

use web::local::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalUntagSamlProviderOutput = OutputWrapper<UntagSamlProviderOutput>;

impl From<LocalUntagSamlProviderOutput> for XmlResponse {
    fn from(val: LocalUntagSamlProviderOutput) -> Self {
        super::confirmation::xml_response("UntagSAMLProviderResponse", &val.request_id)
    }
}
