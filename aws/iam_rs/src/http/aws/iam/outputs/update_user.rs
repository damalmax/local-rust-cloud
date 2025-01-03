use aws_sdk_iam::operation::update_user::UpdateUserOutput;

use web::local::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalUpdateUserOutput = OutputWrapper<UpdateUserOutput>;

impl From<LocalUpdateUserOutput> for XmlResponse {
    fn from(val: LocalUpdateUserOutput) -> Self {
        super::confirmation::xml_response("UpdateUserResponse", &val.request_id)
    }
}
