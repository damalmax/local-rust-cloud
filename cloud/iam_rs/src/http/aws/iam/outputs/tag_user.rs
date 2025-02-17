use aws_sdk_iam::operation::tag_user::TagUserOutput;

use web::local::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalTagUserOutput = OutputWrapper<TagUserOutput>;

impl From<LocalTagUserOutput> for XmlResponse {
    fn from(val: LocalTagUserOutput) -> Self {
        super::confirmation::xml_response("TagUserResponse", &val.request_id)
    }
}
