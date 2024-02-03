use aws_sdk_iam::operation::set_default_policy_version::SetDefaultPolicyVersionOutput;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalSetDefaultPolicyVersionOutput = OutputWrapper<SetDefaultPolicyVersionOutput>;

impl From<LocalSetDefaultPolicyVersionOutput> for XmlResponse {
    fn from(val: LocalSetDefaultPolicyVersionOutput) -> Self {
        super::confirmation::xml_response("SetDefaultPolicyVersionResponse", &val.request_id)
    }
}
