use aws_sdk_iam::operation::delete_instance_profile::DeleteInstanceProfileOutput;

use web::local::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalDeleteInstanceProfileOutput = OutputWrapper<DeleteInstanceProfileOutput>;

impl From<LocalDeleteInstanceProfileOutput> for XmlResponse {
    fn from(val: LocalDeleteInstanceProfileOutput) -> Self {
        super::confirmation::xml_response("DeleteInstanceProfileResponse", &val.request_id)
    }
}
