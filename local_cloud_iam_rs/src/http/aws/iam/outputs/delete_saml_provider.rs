use aws_sdk_iam::operation::delete_saml_provider::DeleteSamlProviderOutput;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalDeleteSamlProviderOutput = OutputWrapper<DeleteSamlProviderOutput>;

impl From<LocalDeleteSamlProviderOutput> for XmlResponse {
    fn from(val: LocalDeleteSamlProviderOutput) -> Self {
        super::confirmation::xml_response("DeleteSAMLProviderResponse", &val.request_id)
    }
}
