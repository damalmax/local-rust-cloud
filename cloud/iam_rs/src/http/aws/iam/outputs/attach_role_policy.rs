use aws_sdk_iam::operation::attach_role_policy::AttachRolePolicyOutput;

use web::local::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalAttachRolePolicyOutput = OutputWrapper<AttachRolePolicyOutput>;

impl From<LocalAttachRolePolicyOutput> for XmlResponse {
    fn from(val: LocalAttachRolePolicyOutput) -> Self {
        super::confirmation::xml_response("AttachRolePolicyResponse", &val.request_id)
    }
}
