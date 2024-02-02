use aws_sdk_iam::operation::add_role_to_instance_profile::AddRoleToInstanceProfileOutput;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalAddRoleToInstanceProfileOutput = OutputWrapper<AddRoleToInstanceProfileOutput>;

impl From<LocalAddRoleToInstanceProfileOutput> for XmlResponse {
    fn from(val: LocalAddRoleToInstanceProfileOutput) -> Self {
        super::confirmation::xml_response("AddRoleToInstanceProfileResponse", &val.request_id)
    }
}
