use aws_sdk_iam::operation::delete_user::DeleteUserOutput;

use web::local::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalDeleteUserOutput = OutputWrapper<DeleteUserOutput>;

impl From<LocalDeleteUserOutput> for XmlResponse {
    fn from(val: LocalDeleteUserOutput) -> Self {
        super::confirmation::xml_response("DeleteUserResponse", &val.request_id)
    }
}
