use aws_sdk_iam::operation::list_ssh_public_keys::ListSshPublicKeysOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_axum::local::web::XmlResponse;
use local_cloud_xml::{write_request_metadata_tag, write_tag_with_value};

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalListSshPublicKeysOutput = OutputWrapper<ListSshPublicKeysOutput>;

impl From<LocalListSshPublicKeysOutput> for XmlResponse {
    fn from(val: LocalListSshPublicKeysOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("ListSSHPublicKeysResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("ListSSHPublicKeysResult").finish();

        super::ssh_public_key_metadata::write_slice(&mut result_tag, "SSHPublicKeys", val.inner.ssh_public_keys());
        if let Some(token) = val.inner.marker() {
            write_tag_with_value(&mut result_tag, "Marker", Some(token));
        }
        write_tag_with_value(&mut result_tag, "IsTruncated", Some(val.inner.is_truncated.to_string()));

        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
