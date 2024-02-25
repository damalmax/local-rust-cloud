use aws_sdk_iam::operation::delete_user_permissions_boundary::DeleteUserPermissionsBoundaryOutput;

use local_cloud_axum::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalDeleteUserPermissionsBoundaryOutput = OutputWrapper<DeleteUserPermissionsBoundaryOutput>;

impl From<LocalDeleteUserPermissionsBoundaryOutput> for XmlResponse {
    fn from(val: LocalDeleteUserPermissionsBoundaryOutput) -> Self {
        super::confirmation::xml_response("DeleteUserPermissionsBoundaryResponse", &val.request_id)
    }
}
