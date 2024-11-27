use aws_sdk_iam::operation::get_account_authorization_details::GetAccountAuthorizationDetailsOutput;
use aws_smithy_xml::encode::XmlWriter;

use web::local::XmlResponse;
use xml::{write_request_metadata_tag, write_tag_with_value};

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalGetAccountAuthorizationDetailsOutput = OutputWrapper<GetAccountAuthorizationDetailsOutput>;

impl From<LocalGetAccountAuthorizationDetailsOutput> for XmlResponse {
    fn from(val: LocalGetAccountAuthorizationDetailsOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("GetAccountAuthorizationDetailsResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("GetAccountAuthorizationDetailsResult").finish();

        super::user_details::write_slice(&mut result_tag, val.inner.user_detail_list());
        super::group_details::write_slice(&mut result_tag, val.inner.group_detail_list());
        super::role_details::write_slice(&mut result_tag, val.inner.role_detail_list());
        super::managed_policy_details::write_slice(&mut result_tag, val.inner.policies());

        if let Some(token) = val.inner.marker() {
            write_tag_with_value(&mut result_tag, "Marker", Some(token));
        }
        write_tag_with_value(&mut result_tag, "IsTruncated", Some(val.inner.is_truncated.to_string()));
        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
