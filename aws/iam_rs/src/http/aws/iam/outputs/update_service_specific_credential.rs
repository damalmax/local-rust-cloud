use aws_sdk_iam::operation::update_service_specific_credential::UpdateServiceSpecificCredentialOutput;

use web::local::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalUpdateServiceSpecificCredentialOutput = OutputWrapper<UpdateServiceSpecificCredentialOutput>;

impl From<LocalUpdateServiceSpecificCredentialOutput> for XmlResponse {
    fn from(val: LocalUpdateServiceSpecificCredentialOutput) -> Self {
        super::confirmation::xml_response("UpdateServiceSpecificCredentialResponse", &val.request_id)
    }
}
