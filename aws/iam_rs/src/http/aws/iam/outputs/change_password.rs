use aws_sdk_iam::operation::change_password::ChangePasswordOutput;

use web::local::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalChangePasswordOutput = OutputWrapper<ChangePasswordOutput>;

impl From<LocalChangePasswordOutput> for XmlResponse {
    fn from(val: LocalChangePasswordOutput) -> Self {
        super::confirmation::xml_response("ChangePasswordResponse", &val.request_id)
    }
}
