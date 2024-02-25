use aws_sdk_iam::operation::remove_user_from_group::RemoveUserFromGroupOutput;

use local_cloud_axum::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalRemoveUserFromGroupOutput = OutputWrapper<RemoveUserFromGroupOutput>;

impl From<LocalRemoveUserFromGroupOutput> for XmlResponse {
    fn from(val: LocalRemoveUserFromGroupOutput) -> Self {
        super::confirmation::xml_response("RemoveUserFromGroupResponse", &val.request_id)
    }
}
