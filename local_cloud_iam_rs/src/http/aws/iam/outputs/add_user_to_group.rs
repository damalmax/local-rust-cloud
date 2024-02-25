use aws_sdk_iam::operation::add_user_to_group::AddUserToGroupOutput;

use local_cloud_axum::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalAddUserToGroupOutput = OutputWrapper<AddUserToGroupOutput>;

impl From<LocalAddUserToGroupOutput> for XmlResponse {
    fn from(val: LocalAddUserToGroupOutput) -> Self {
        super::confirmation::xml_response("AddUserToGroupResponse", &val.request_id)
    }
}
