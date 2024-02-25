use aws_sdk_iam::operation::get_account_summary::GetAccountSummaryOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_axum::local::web::XmlResponse;
use local_cloud_xml::{write_request_metadata_tag, write_tag_with_value};

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalGetAccountSummaryOutput = OutputWrapper<GetAccountSummaryOutput>;

impl From<LocalGetAccountSummaryOutput> for XmlResponse {
    fn from(val: LocalGetAccountSummaryOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("GetAccountSummaryResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("GetAccountSummaryResult").finish();

        let mut summary_map_tag = result_tag.start_el("SummaryMap").finish();
        if let Some(map) = val.inner.summary_map() {
            for (key, value) in map {
                let mut entry_tag = summary_map_tag.start_el("entry").finish();
                write_tag_with_value(&mut entry_tag, "key", Some(key.as_str()));
                write_tag_with_value(&mut entry_tag, "value", Some(value.to_string()));
                entry_tag.finish();
            }
        }

        summary_map_tag.finish();
        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
