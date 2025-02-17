use aws_sdk_iam::operation::delete_login_profile::DeleteLoginProfileOutput;

use web::local::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalDeleteLoginProfileOutput = OutputWrapper<DeleteLoginProfileOutput>;

impl From<LocalDeleteLoginProfileOutput> for XmlResponse {
    fn from(val: LocalDeleteLoginProfileOutput) -> Self {
        super::confirmation::xml_response("DeleteLoginProfileResponse", &val.request_id)
    }
}
