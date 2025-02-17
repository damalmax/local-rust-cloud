use aws_sdk_iam::types::AccessKeyMetadata;
use aws_smithy_xml::encode::ScopeWriter;

use xml::{write_iso8061_datetime_value_tag, write_tag_with_value};

pub(crate) fn write_slice(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, items: &[AccessKeyMetadata]) {
    let mut items_tag = parent_tag.start_el(wrapper_tag_name).finish();
    for item in items {
        write(&mut items_tag, "member", item);
    }
    items_tag.finish()
}

pub(crate) fn write(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, item: &AccessKeyMetadata) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    write_tag_with_value(&mut wrapper_tag, "UserName", item.user_name());
    write_tag_with_value(&mut wrapper_tag, "AccessKeyId", item.access_key_id());
    write_tag_with_value(&mut wrapper_tag, "Status", item.status().map(|v| v.as_str()));
    write_iso8061_datetime_value_tag(&mut wrapper_tag, "CreateDate", item.create_date());
    wrapper_tag.finish();
}
