use aws_sdk_iam::operation::untag_role::UntagRoleOutput;

use local_cloud_axum::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalUntagRoleOutput = OutputWrapper<UntagRoleOutput>;

impl From<LocalUntagRoleOutput> for XmlResponse {
    fn from(val: LocalUntagRoleOutput) -> Self {
        super::confirmation::xml_response("UntagRoleResponse", &val.request_id)
    }
}
