use aws_sdk_iam::operation::put_user_policy::PutUserPolicyOutput;

use web::local::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalPutUserPolicyOutput = OutputWrapper<PutUserPolicyOutput>;

impl From<LocalPutUserPolicyOutput> for XmlResponse {
    fn from(val: LocalPutUserPolicyOutput) -> Self {
        super::confirmation::xml_response("PutUserPolicyResponse", &val.request_id)
    }
}
