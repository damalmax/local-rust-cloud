use aws_sdk_iam::types::ServerCertificateMetadata;
use aws_smithy_xml::encode::ScopeWriter;

use xml::{write_iso8061_datetime_value_tag, write_tag_with_value};

pub(crate) fn write_slice(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, items: &[ServerCertificateMetadata]) {
    let mut items_tag = parent_tag.start_el(wrapper_tag_name).finish();
    for item in items {
        write(&mut items_tag, "member", item);
    }
    items_tag.finish()
}

pub(crate) fn write(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, item: &ServerCertificateMetadata) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    write_tag_with_value(&mut wrapper_tag, "Path", Some(item.path()));
    write_tag_with_value(&mut wrapper_tag, "ServerCertificateName", Some(item.server_certificate_name()));
    write_tag_with_value(&mut wrapper_tag, "ServerCertificateId", Some(item.server_certificate_id()));
    write_tag_with_value(&mut wrapper_tag, "Arn", Some(item.arn()));
    write_iso8061_datetime_value_tag(&mut wrapper_tag, "UploadDate", item.upload_date());
    write_iso8061_datetime_value_tag(&mut wrapper_tag, "Expiration", item.expiration());
    wrapper_tag.finish();
}
