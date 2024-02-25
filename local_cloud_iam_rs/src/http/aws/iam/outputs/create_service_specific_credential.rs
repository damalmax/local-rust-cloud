use aws_sdk_iam::operation::create_service_specific_credential::CreateServiceSpecificCredentialOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_axum::local::web::XmlResponse;
use local_cloud_xml::write_request_metadata_tag;

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalCreateServiceSpecificCredentialOutput = OutputWrapper<CreateServiceSpecificCredentialOutput>;

impl From<LocalCreateServiceSpecificCredentialOutput> for XmlResponse {
    fn from(val: LocalCreateServiceSpecificCredentialOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("CreateServiceSpecificCredentialResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("CreateServiceSpecificCredentialResult").finish();

        if let Some(credential) = val.inner.service_specific_credential() {
            super::service_specific_credentials::write(&mut result_tag, "ServiceSpecificCredential", credential);
        }

        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
