use aws_sdk_iam::operation::list_open_id_connect_providers::ListOpenIdConnectProvidersOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_axum::local::web::XmlResponse;
use local_cloud_xml::write_request_metadata_tag;

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalListOpenIdConnectProvidersOutput = OutputWrapper<ListOpenIdConnectProvidersOutput>;

impl From<LocalListOpenIdConnectProvidersOutput> for XmlResponse {
    fn from(val: LocalListOpenIdConnectProvidersOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("ListOpenIDConnectProvidersResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("ListOpenIDConnectProvidersResult").finish();

        super::open_id_connect_providers::write_slice(
            &mut result_tag,
            "OpenIDConnectProviderList",
            val.inner.open_id_connect_provider_list(),
        );
        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
