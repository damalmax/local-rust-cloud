use aws_sdk_iam::types::ServerCertificate;
use aws_smithy_xml::encode::ScopeWriter;

use local_cloud_xml::write_tag_with_value;

pub(crate) fn write_slice(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, items: &[ServerCertificate]) {
    let mut items_tag = parent_tag.start_el(wrapper_tag_name).finish();
    for item in items {
        write(&mut items_tag, "member", item);
    }
    items_tag.finish()
}

pub(crate) fn write(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, item: &ServerCertificate) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    if let Some(certificate_metadata) = item.server_certificate_metadata() {
        super::server_certificate_metadata::write(&mut wrapper_tag, "ServerCertificateMetadata", certificate_metadata);
    }
    write_tag_with_value(&mut wrapper_tag, "CertificateBody", Some(item.certificate_body()));
    write_tag_with_value(&mut wrapper_tag, "CertificateChain", item.certificate_chain());
    super::tags::write_slice(&mut wrapper_tag, item.tags());
    wrapper_tag.finish();
}
