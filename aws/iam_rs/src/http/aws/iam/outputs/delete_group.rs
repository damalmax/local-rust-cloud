use aws_sdk_iam::operation::delete_group::DeleteGroupOutput;

use web::local::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalDeleteGroupOutput = OutputWrapper<DeleteGroupOutput>;

impl From<LocalDeleteGroupOutput> for XmlResponse {
    fn from(val: LocalDeleteGroupOutput) -> Self {
        super::confirmation::xml_response("DeleteGroupResponse", &val.request_id)
    }
}
