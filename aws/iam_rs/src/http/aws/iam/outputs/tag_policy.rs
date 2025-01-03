use aws_sdk_iam::operation::tag_policy::TagPolicyOutput;

use web::local::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalTagPolicyOutput = OutputWrapper<TagPolicyOutput>;

impl From<LocalTagPolicyOutput> for XmlResponse {
    fn from(val: LocalTagPolicyOutput) -> Self {
        super::confirmation::xml_response("TagPolicyResponse", &val.request_id)
    }
}
