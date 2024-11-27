use aws_sdk_iam::operation::remove_role_from_instance_profile::RemoveRoleFromInstanceProfileOutput;

use web::local::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalRemoveRoleFromInstanceProfileOutput = OutputWrapper<RemoveRoleFromInstanceProfileOutput>;

impl From<LocalRemoveRoleFromInstanceProfileOutput> for XmlResponse {
    fn from(val: LocalRemoveRoleFromInstanceProfileOutput) -> Self {
        super::confirmation::xml_response("RemoveRoleFromInstanceProfileResponse", &val.request_id)
    }
}
