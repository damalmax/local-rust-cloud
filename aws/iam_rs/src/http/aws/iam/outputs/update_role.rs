use aws_sdk_iam::operation::update_role::UpdateRoleOutput;

use web::local::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalUpdateRoleOutput = OutputWrapper<UpdateRoleOutput>;

impl From<LocalUpdateRoleOutput> for XmlResponse {
    fn from(val: LocalUpdateRoleOutput) -> Self {
        super::confirmation::xml_response("UpdateRoleResponse", &val.request_id)
    }
}
