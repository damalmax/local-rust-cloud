use aws_sdk_iam::types::ErrorDetails;
use aws_smithy_xml::encode::ScopeWriter;

use local_cloud_xml::write_tag_with_value;

pub(crate) fn write_slice(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, items: &[ErrorDetails]) {
    let mut items_tag = parent_tag.start_el(wrapper_tag_name).finish();
    for item in items {
        write(&mut items_tag, "member", item);
    }
    items_tag.finish()
}

pub(crate) fn write(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, item: &ErrorDetails) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    write_tag_with_value(&mut wrapper_tag, "Message", Some(item.message()));
    write_tag_with_value(&mut wrapper_tag, "Code", Some(item.code()));
    wrapper_tag.finish();
}
