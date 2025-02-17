use aws_sdk_iam::operation::put_group_policy::PutGroupPolicyOutput;

use web::local::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalPutGroupPolicyOutput = OutputWrapper<PutGroupPolicyOutput>;

impl From<LocalPutGroupPolicyOutput> for XmlResponse {
    fn from(val: LocalPutGroupPolicyOutput) -> Self {
        super::confirmation::xml_response("PutGroupPolicyResponse", &val.request_id)
    }
}
