use aws_sdk_iam::types::Statement;
use aws_smithy_xml::encode::ScopeWriter;

use xml::write_tag_with_value;

pub(crate) fn write_slice(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, items: &[Statement]) {
    let mut items_tag = parent_tag.start_el(wrapper_tag_name).finish();
    for item in items {
        write(&mut items_tag, "member", item);
    }
    items_tag.finish()
}

pub(crate) fn write(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, item: &Statement) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    write_tag_with_value(&mut wrapper_tag, "SourcePolicyId", item.source_policy_id());
    write_tag_with_value(&mut wrapper_tag, "SourcePolicyType", item.source_policy_type().map(|v| v.as_str()));
    if let Some(start_position) = item.start_position() {
        super::positions::write(&mut wrapper_tag, "StartPosition", start_position);
    }
    if let Some(end_position) = item.end_position() {
        super::positions::write(&mut wrapper_tag, "EndPosition", end_position);
    }
    wrapper_tag.finish();
}
