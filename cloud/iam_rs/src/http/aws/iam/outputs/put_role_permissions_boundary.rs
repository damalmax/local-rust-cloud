use aws_sdk_iam::operation::put_role_permissions_boundary::PutRolePermissionsBoundaryOutput;

use web::local::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalPutRolePermissionsBoundaryOutput = OutputWrapper<PutRolePermissionsBoundaryOutput>;

impl From<LocalPutRolePermissionsBoundaryOutput> for XmlResponse {
    fn from(val: LocalPutRolePermissionsBoundaryOutput) -> Self {
        super::confirmation::xml_response("PutRolePermissionsBoundaryResponse", &val.request_id)
    }
}
