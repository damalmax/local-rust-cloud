use aws_sdk_iam::operation::delete_service_specific_credential::DeleteServiceSpecificCredentialOutput;

use web::local::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalDeleteServiceSpecificCredentialOutput = OutputWrapper<DeleteServiceSpecificCredentialOutput>;

impl From<LocalDeleteServiceSpecificCredentialOutput> for XmlResponse {
    fn from(val: LocalDeleteServiceSpecificCredentialOutput) -> Self {
        super::confirmation::xml_response("DeleteServiceSpecificCredentialResponse", &val.request_id)
    }
}
