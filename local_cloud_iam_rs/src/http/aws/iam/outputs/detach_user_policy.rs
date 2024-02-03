use aws_sdk_iam::operation::detach_user_policy::DetachUserPolicyOutput;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalDetachUserPolicyOutput = OutputWrapper<DetachUserPolicyOutput>;

impl From<LocalDetachUserPolicyOutput> for XmlResponse {
    fn from(val: LocalDetachUserPolicyOutput) -> Self {
        super::confirmation::xml_response("DetachUserPolicyResponse", &val.request_id)
    }
}
