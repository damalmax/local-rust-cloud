use aws_sdk_iam::types::TrackedActionLastAccessed;
use aws_smithy_xml::encode::ScopeWriter;

use xml::{write_iso8061_datetime_value_tag, write_tag_with_value};

pub(crate) fn write_slice(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, items: &[TrackedActionLastAccessed]) {
    let mut items_tag = parent_tag.start_el(wrapper_tag_name).finish();
    for item in items {
        write(&mut items_tag, "member", item);
    }
    items_tag.finish()
}

pub(crate) fn write(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, item: &TrackedActionLastAccessed) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    write_tag_with_value(&mut wrapper_tag, "ActionName", item.action_name());
    write_tag_with_value(&mut wrapper_tag, "LastAccessedEntity", item.last_accessed_entity());
    write_iso8061_datetime_value_tag(&mut wrapper_tag, "LastAccessedTime", item.last_accessed_time());
    write_tag_with_value(&mut wrapper_tag, "LastAccessedRegion", item.last_accessed_region());
    wrapper_tag.finish();
}
