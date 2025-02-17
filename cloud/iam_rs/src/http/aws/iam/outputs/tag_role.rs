use aws_sdk_iam::operation::tag_role::TagRoleOutput;

use web::local::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalTagRoleOutput = OutputWrapper<TagRoleOutput>;

impl From<LocalTagRoleOutput> for XmlResponse {
    fn from(val: LocalTagRoleOutput) -> Self {
        super::confirmation::xml_response("TagRoleResponse", &val.request_id)
    }
}
