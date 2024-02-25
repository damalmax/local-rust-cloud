use aws_sdk_iam::operation::delete_role_policy::DeleteRolePolicyOutput;

use local_cloud_axum::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalDeleteRolePolicyOutput = OutputWrapper<DeleteRolePolicyOutput>;

impl From<LocalDeleteRolePolicyOutput> for XmlResponse {
    fn from(val: LocalDeleteRolePolicyOutput) -> Self {
        super::confirmation::xml_response("DeleteRolePolicyResponse", &val.request_id)
    }
}
