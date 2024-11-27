use aws_sdk_iam::operation::remove_client_id_from_open_id_connect_provider::RemoveClientIdFromOpenIdConnectProviderOutput;

use web::local::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalRemoveClientIdFromOpenIdConnectProviderOutput =
    OutputWrapper<RemoveClientIdFromOpenIdConnectProviderOutput>;

impl From<LocalRemoveClientIdFromOpenIdConnectProviderOutput> for XmlResponse {
    fn from(val: LocalRemoveClientIdFromOpenIdConnectProviderOutput) -> Self {
        super::confirmation::xml_response("RemoveClientIDFromOpenIDConnectProviderResponse", &val.request_id)
    }
}
