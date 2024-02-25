use aws_sdk_iam::operation::delete_user_policy::DeleteUserPolicyOutput;

use local_cloud_axum::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalDeleteUserPolicyOutput = OutputWrapper<DeleteUserPolicyOutput>;

impl From<LocalDeleteUserPolicyOutput> for XmlResponse {
    fn from(val: LocalDeleteUserPolicyOutput) -> Self {
        super::confirmation::xml_response("DeleteUserPolicyResponse", &val.request_id)
    }
}
