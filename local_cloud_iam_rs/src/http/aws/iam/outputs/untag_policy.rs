use aws_sdk_iam::operation::untag_policy::UntagPolicyOutput;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalUntagPolicyOutput = OutputWrapper<UntagPolicyOutput>;

impl From<LocalUntagPolicyOutput> for XmlResponse {
    fn from(val: LocalUntagPolicyOutput) -> Self {
        super::confirmation::xml_response("UntagPolicyResponse", &val.request_id)
    }
}
