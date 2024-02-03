use aws_sdk_iam::operation::delete_group_policy::DeleteGroupPolicyOutput;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalDeleteGroupPolicyOutput = OutputWrapper<DeleteGroupPolicyOutput>;

impl From<LocalDeleteGroupPolicyOutput> for XmlResponse {
    fn from(val: LocalDeleteGroupPolicyOutput) -> Self {
        super::confirmation::xml_response("DeleteGroupPolicyResponse", &val.request_id)
    }
}
