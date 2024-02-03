use aws_sdk_iam::operation::detach_role_policy::DetachRolePolicyOutput;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalDetachRolePolicyOutput = OutputWrapper<DetachRolePolicyOutput>;

impl From<LocalDetachRolePolicyOutput> for XmlResponse {
    fn from(val: LocalDetachRolePolicyOutput) -> Self {
        super::confirmation::xml_response("DetachRolePolicyResponse", &val.request_id)
    }
}
