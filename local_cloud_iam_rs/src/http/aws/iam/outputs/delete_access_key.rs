use aws_sdk_iam::operation::delete_access_key::DeleteAccessKeyOutput;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalDeleteAccessKeyOutput = OutputWrapper<DeleteAccessKeyOutput>;

impl From<LocalDeleteAccessKeyOutput> for XmlResponse {
    fn from(val: LocalDeleteAccessKeyOutput) -> Self {
        super::confirmation::xml_response("DeleteAccessKeyResponse", &val.request_id)
    }
}
