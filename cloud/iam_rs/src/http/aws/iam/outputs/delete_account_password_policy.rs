use aws_sdk_iam::operation::delete_account_password_policy::DeleteAccountPasswordPolicyOutput;

use web::local::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalDeleteAccountPasswordPolicyOutput = OutputWrapper<DeleteAccountPasswordPolicyOutput>;

impl From<LocalDeleteAccountPasswordPolicyOutput> for XmlResponse {
    fn from(val: LocalDeleteAccountPasswordPolicyOutput) -> Self {
        super::confirmation::xml_response("DeleteAccountPasswordPolicyResponse", &val.request_id)
    }
}
