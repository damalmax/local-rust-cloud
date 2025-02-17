use aws_sdk_iam::operation::delete_policy_version::DeletePolicyVersionOutput;

use web::local::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalDeletePolicyVersionOutput = OutputWrapper<DeletePolicyVersionOutput>;

impl From<LocalDeletePolicyVersionOutput> for XmlResponse {
    fn from(val: LocalDeletePolicyVersionOutput) -> Self {
        super::confirmation::xml_response("DeletePolicyVersionResponse", &val.request_id)
    }
}
