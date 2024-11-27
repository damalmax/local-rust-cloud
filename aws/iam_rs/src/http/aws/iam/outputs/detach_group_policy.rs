use aws_sdk_iam::operation::detach_group_policy::DetachGroupPolicyOutput;

use web::local::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalDetachGroupPolicyOutput = OutputWrapper<DetachGroupPolicyOutput>;

impl From<LocalDetachGroupPolicyOutput> for XmlResponse {
    fn from(val: LocalDetachGroupPolicyOutput) -> Self {
        super::confirmation::xml_response("DetachGroupPolicyResponse", &val.request_id)
    }
}
