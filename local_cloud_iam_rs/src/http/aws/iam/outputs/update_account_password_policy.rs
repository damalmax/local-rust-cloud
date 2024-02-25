use aws_sdk_iam::operation::update_account_password_policy::UpdateAccountPasswordPolicyOutput;

use local_cloud_axum::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalUpdateAccountPasswordPolicyOutput = OutputWrapper<UpdateAccountPasswordPolicyOutput>;

impl From<LocalUpdateAccountPasswordPolicyOutput> for XmlResponse {
    fn from(val: LocalUpdateAccountPasswordPolicyOutput) -> Self {
        super::confirmation::xml_response("UpdateAccountPasswordPolicyResponse", &val.request_id)
    }
}
