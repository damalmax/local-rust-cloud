use aws_sdk_iam::operation::add_client_id_to_open_id_connect_provider::AddClientIdToOpenIdConnectProviderOutput;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalAddClientIdToOpenIdConnectProviderOutput = OutputWrapper<AddClientIdToOpenIdConnectProviderOutput>;

impl From<LocalAddClientIdToOpenIdConnectProviderOutput> for XmlResponse {
    fn from(val: LocalAddClientIdToOpenIdConnectProviderOutput) -> Self {
        super::confirmation::xml_response("AddClientIDToOpenIDConnectProviderResponse", &val.request_id)
    }
}
