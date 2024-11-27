use aws_sdk_iam::operation::update_login_profile::UpdateLoginProfileOutput;

use web::local::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalUpdateLoginProfileOutput = OutputWrapper<UpdateLoginProfileOutput>;

impl From<LocalUpdateLoginProfileOutput> for XmlResponse {
    fn from(val: LocalUpdateLoginProfileOutput) -> Self {
        super::confirmation::xml_response("UpdateLoginProfileResponse", &val.request_id)
    }
}
