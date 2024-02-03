use aws_sdk_iam::operation::put_role_policy::PutRolePolicyOutput;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalPutRolePolicyOutput = OutputWrapper<PutRolePolicyOutput>;

impl From<LocalPutRolePolicyOutput> for XmlResponse {
    fn from(val: LocalPutRolePolicyOutput) -> Self {
        super::confirmation::xml_response("PutRolePolicyResponse", &val.request_id)
    }
}
