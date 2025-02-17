use aws_sdk_iam::types::EntityDetails;
use aws_smithy_xml::encode::ScopeWriter;

use xml::write_iso8061_datetime_value_tag;

pub(crate) fn write_slice(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, items: &[EntityDetails]) {
    let mut items_tag = parent_tag.start_el(wrapper_tag_name).finish();
    for item in items {
        write(&mut items_tag, "member", item);
    }
    items_tag.finish()
}

pub(crate) fn write(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, item: &EntityDetails) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    if let Some(entity_info) = item.entity_info() {
        super::entity_info::write(&mut wrapper_tag, "EntityInfo", entity_info);
    }
    write_iso8061_datetime_value_tag(&mut wrapper_tag, "LastAuthenticated", item.last_authenticated());
    wrapper_tag.finish();
}
