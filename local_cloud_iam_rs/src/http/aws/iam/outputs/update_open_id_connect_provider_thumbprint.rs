use aws_sdk_iam::operation::update_open_id_connect_provider_thumbprint::UpdateOpenIdConnectProviderThumbprintOutput;

use local_cloud_axum::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalUpdateOpenIdConnectProviderThumbprintOutput = OutputWrapper<UpdateOpenIdConnectProviderThumbprintOutput>;

impl From<LocalUpdateOpenIdConnectProviderThumbprintOutput> for XmlResponse {
    fn from(val: LocalUpdateOpenIdConnectProviderThumbprintOutput) -> Self {
        super::confirmation::xml_response("UpdateOpenIdConnectProviderThumbprintResponse", &val.request_id)
    }
}
