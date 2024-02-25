use aws_sdk_iam::operation::update_ssh_public_key::UpdateSshPublicKeyOutput;

use local_cloud_axum::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalUpdateSshPublicKeyOutput = OutputWrapper<UpdateSshPublicKeyOutput>;

impl From<LocalUpdateSshPublicKeyOutput> for XmlResponse {
    fn from(val: LocalUpdateSshPublicKeyOutput) -> Self {
        super::confirmation::xml_response("UpdateSSHPublicKeyResponse", &val.request_id)
    }
}
