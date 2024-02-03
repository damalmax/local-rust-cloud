use aws_sdk_iam::operation::put_user_permissions_boundary::PutUserPermissionsBoundaryOutput;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalPutUserPermissionsBoundaryOutput = OutputWrapper<PutUserPermissionsBoundaryOutput>;

impl From<LocalPutUserPermissionsBoundaryOutput> for XmlResponse {
    fn from(val: LocalPutUserPermissionsBoundaryOutput) -> Self {
        super::confirmation::xml_response("PutUserPermissionsBoundaryResponse", &val.request_id)
    }
}
