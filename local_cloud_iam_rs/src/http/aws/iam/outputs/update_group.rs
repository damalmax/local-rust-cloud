use aws_sdk_iam::operation::update_group::UpdateGroupOutput;

use local_cloud_axum::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalUpdateGroupOutput = OutputWrapper<UpdateGroupOutput>;

impl From<LocalUpdateGroupOutput> for XmlResponse {
    fn from(val: LocalUpdateGroupOutput) -> Self {
        super::confirmation::xml_response("UpdateGroupResponse", &val.request_id)
    }
}
