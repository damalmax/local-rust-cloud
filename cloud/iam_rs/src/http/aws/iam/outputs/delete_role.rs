use aws_sdk_iam::operation::delete_role::DeleteRoleOutput;

use web::local::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalDeleteRoleOutput = OutputWrapper<DeleteRoleOutput>;

impl From<LocalDeleteRoleOutput> for XmlResponse {
    fn from(val: LocalDeleteRoleOutput) -> Self {
        super::confirmation::xml_response("DeleteRoleResponse", &val.request_id)
    }
}
