use aws_sdk_iam::operation::tag_saml_provider::TagSamlProviderOutput;

use local_cloud_axum::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalTagSamlProviderOutput = OutputWrapper<TagSamlProviderOutput>;

impl From<LocalTagSamlProviderOutput> for XmlResponse {
    fn from(val: LocalTagSamlProviderOutput) -> Self {
        super::confirmation::xml_response("TagSAMLProviderResponse", &val.request_id)
    }
}
