use aws_sdk_iam::operation::delete_role_permissions_boundary::DeleteRolePermissionsBoundaryOutput;

use local_cloud_axum::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalDeleteRolePermissionsBoundaryOutput = OutputWrapper<DeleteRolePermissionsBoundaryOutput>;

impl From<LocalDeleteRolePermissionsBoundaryOutput> for XmlResponse {
    fn from(val: LocalDeleteRolePermissionsBoundaryOutput) -> Self {
        super::confirmation::xml_response("DeleteRolePermissionsBoundaryResponse", &val.request_id)
    }
}
