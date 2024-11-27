use aws_sdk_iam::operation::delete_policy::DeletePolicyOutput;

use web::local::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalDeletePolicyOutput = OutputWrapper<DeletePolicyOutput>;

impl From<LocalDeletePolicyOutput> for XmlResponse {
    fn from(val: LocalDeletePolicyOutput) -> Self {
        super::confirmation::xml_response("DeletePolicyResponse", &val.request_id)
    }
}
