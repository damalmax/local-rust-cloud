use aws_sdk_iam::operation::get_ssh_public_key::GetSshPublicKeyOutput;
use aws_smithy_xml::encode::XmlWriter;

use web::local::XmlResponse;
use xml::write_request_metadata_tag;

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalGetSshPublicKeyOutput = OutputWrapper<GetSshPublicKeyOutput>;

impl From<LocalGetSshPublicKeyOutput> for XmlResponse {
    fn from(val: LocalGetSshPublicKeyOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("GetSSHPublicKeyResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("GetSSHPublicKeyResult").finish();
        if let Some(ssh_public_key) = val.inner.ssh_public_key() {
            super::ssh_public_keys::write(&mut result_tag, "SSHPublicKey", ssh_public_key);
        }
        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
