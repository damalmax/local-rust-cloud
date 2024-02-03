use aws_sdk_iam::types::SshPublicKey;
use aws_smithy_xml::encode::ScopeWriter;

pub(crate) fn write_slice(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, items: &[SshPublicKey]) {
    let mut items_tag = parent_tag.start_el(wrapper_tag_name).finish();
    for item in items {
        write(&mut items_tag, "member", item);
    }
    items_tag.finish()
}

pub(crate) fn write(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, item: &SshPublicKey) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();

    wrapper_tag.finish();
}
