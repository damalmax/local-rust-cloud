use aws_sdk_iam::operation::attach_group_policy::AttachGroupPolicyOutput;

use local_cloud_axum::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalAttachGroupPolicyOutput = OutputWrapper<AttachGroupPolicyOutput>;

impl From<LocalAttachGroupPolicyOutput> for XmlResponse {
    fn from(val: LocalAttachGroupPolicyOutput) -> Self {
        super::confirmation::xml_response("AttachGroupPolicyResponse", &val.request_id)
    }
}
