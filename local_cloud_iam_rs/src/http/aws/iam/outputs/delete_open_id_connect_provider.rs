use aws_sdk_iam::operation::delete_open_id_connect_provider::DeleteOpenIdConnectProviderOutput;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalDeleteOpenIdConnectProviderOutput = OutputWrapper<DeleteOpenIdConnectProviderOutput>;

impl From<LocalDeleteOpenIdConnectProviderOutput> for XmlResponse {
    fn from(val: LocalDeleteOpenIdConnectProviderOutput) -> Self {
        super::confirmation::xml_response("DeleteOpenIDConnectProviderResponse", &val.request_id)
    }
}
