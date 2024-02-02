use aws_sdk_iam::operation::attach_user_policy::AttachUserPolicyOutput;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalAttachUserPolicyOutput = OutputWrapper<AttachUserPolicyOutput>;

impl From<LocalAttachUserPolicyOutput> for XmlResponse {
    fn from(val: LocalAttachUserPolicyOutput) -> Self {
        super::confirmation::xml_response("AttachUserPolicyResponse", &val.request_id)
    }
}
