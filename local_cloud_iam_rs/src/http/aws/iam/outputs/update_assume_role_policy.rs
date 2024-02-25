use aws_sdk_iam::operation::update_assume_role_policy::UpdateAssumeRolePolicyOutput;

use local_cloud_axum::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalUpdateAssumeRolePolicyOutput = OutputWrapper<UpdateAssumeRolePolicyOutput>;

impl From<LocalUpdateAssumeRolePolicyOutput> for XmlResponse {
    fn from(val: LocalUpdateAssumeRolePolicyOutput) -> Self {
        super::confirmation::xml_response("UpdateAssumeRolePolicyResponse", &val.request_id)
    }
}
