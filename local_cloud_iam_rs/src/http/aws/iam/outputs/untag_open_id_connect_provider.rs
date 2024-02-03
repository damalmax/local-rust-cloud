use aws_sdk_iam::operation::untag_open_id_connect_provider::UntagOpenIdConnectProviderOutput;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalUntagOpenIdConnectProviderOutput = OutputWrapper<UntagOpenIdConnectProviderOutput>;

impl From<LocalUntagOpenIdConnectProviderOutput> for XmlResponse {
    fn from(val: LocalUntagOpenIdConnectProviderOutput) -> Self {
        super::confirmation::xml_response("UntagOpenIdConnectProviderResponse", &val.request_id)
    }
}
