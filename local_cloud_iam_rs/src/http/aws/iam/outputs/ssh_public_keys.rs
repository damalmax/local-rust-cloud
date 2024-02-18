use aws_sdk_iam::types::SshPublicKey;
use aws_smithy_xml::encode::ScopeWriter;

use local_cloud_xml::{write_iso8061_datetime_value_tag, write_tag_with_value};

pub(crate) fn write_slice(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, items: &[SshPublicKey]) {
    let mut items_tag = parent_tag.start_el(wrapper_tag_name).finish();
    for item in items {
        write(&mut items_tag, "member", item);
    }
    items_tag.finish()
}

pub(crate) fn write(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, item: &SshPublicKey) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    write_tag_with_value(&mut wrapper_tag, "UserName", Some(item.user_name()));
    write_tag_with_value(&mut wrapper_tag, "SSHPublicKeyID", Some(item.ssh_public_key_id()));
    write_tag_with_value(&mut wrapper_tag, "SSHPublicKeyBody", Some(item.ssh_public_key_body()));
    write_tag_with_value(&mut wrapper_tag, "Fingerprint", Some(item.fingerprint()));
    write_tag_with_value(&mut wrapper_tag, "Status", Some(item.status().as_str()));
    write_iso8061_datetime_value_tag(&mut wrapper_tag, "UploadDate", item.upload_date());
    wrapper_tag.finish();
}
