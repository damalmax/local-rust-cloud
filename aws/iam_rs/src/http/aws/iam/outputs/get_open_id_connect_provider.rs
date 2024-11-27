use aws_sdk_iam::operation::get_open_id_connect_provider::GetOpenIdConnectProviderOutput;
use aws_smithy_xml::encode::XmlWriter;

use web::local::XmlResponse;
use xml::{write_iso8061_datetime_value_tag, write_request_metadata_tag, write_tag_with_value};

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalGetOpenIdConnectProviderOutput = OutputWrapper<GetOpenIdConnectProviderOutput>;

impl From<LocalGetOpenIdConnectProviderOutput> for XmlResponse {
    fn from(val: LocalGetOpenIdConnectProviderOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("GetOpenIDConnectProviderResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("GetOpenIDConnectProviderResult").finish();

        write_iso8061_datetime_value_tag(&mut result_tag, "CreateDate", val.inner.create_date());
        write_tag_with_value(&mut result_tag, "Url", val.inner.url());

        let mut thumbprints_tag = result_tag.start_el("ThumbprintList").finish();
        for member in val.inner.thumbprint_list() {
            write_tag_with_value(&mut thumbprints_tag, "member", Some(member));
        }
        thumbprints_tag.finish();

        let mut client_ids_tag = result_tag.start_el("ClientIDList").finish();
        for member in val.inner.client_id_list() {
            write_tag_with_value(&mut client_ids_tag, "member", Some(member));
        }
        client_ids_tag.finish();

        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
