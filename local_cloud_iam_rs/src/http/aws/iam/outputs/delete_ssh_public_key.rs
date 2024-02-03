use aws_sdk_iam::operation::delete_ssh_public_key::DeleteSshPublicKeyOutput;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalDeleteSshPublicKeyOutput = OutputWrapper<DeleteSshPublicKeyOutput>;

impl From<LocalDeleteSshPublicKeyOutput> for XmlResponse {
    fn from(val: LocalDeleteSshPublicKeyOutput) -> Self {
        super::confirmation::xml_response("DeleteSSHPublicKeyResponse", &val.request_id)
    }
}
