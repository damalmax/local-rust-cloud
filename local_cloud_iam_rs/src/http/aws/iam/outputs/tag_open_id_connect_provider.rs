use aws_sdk_iam::operation::tag_open_id_connect_provider::TagOpenIdConnectProviderOutput;

use local_cloud_axum::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalTagOpenIdConnectProviderOutput = OutputWrapper<TagOpenIdConnectProviderOutput>;

impl From<LocalTagOpenIdConnectProviderOutput> for XmlResponse {
    fn from(val: LocalTagOpenIdConnectProviderOutput) -> Self {
        super::confirmation::xml_response("TagOpenIdConnectProviderResponse", &val.request_id)
    }
}
