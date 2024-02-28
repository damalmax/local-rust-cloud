use aws_sdk_iam::operation::list_service_specific_credentials::ListServiceSpecificCredentialsOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_axum::local::web::XmlResponse;
use local_cloud_xml::write_request_metadata_tag;

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalListServiceSpecificCredentialsOutput = OutputWrapper<ListServiceSpecificCredentialsOutput>;

impl From<LocalListServiceSpecificCredentialsOutput> for XmlResponse {
    fn from(val: LocalListServiceSpecificCredentialsOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("ListServiceSpecificCredentialsResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("ListServiceSpecificCredentialsResult").finish();

        super::service_specific_credentials_metadata::write_slice(
            &mut result_tag,
            "ServiceSpecificCredentials",
            val.inner.service_specific_credentials(),
        );
        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
