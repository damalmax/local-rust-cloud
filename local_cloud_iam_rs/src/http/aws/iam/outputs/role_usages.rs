use aws_sdk_iam::types::RoleUsageType;
use aws_smithy_xml::encode::ScopeWriter;

use local_cloud_xml::write_tag_with_value;

pub(crate) fn write_slice(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, items: &[RoleUsageType]) {
    let mut items_tag = parent_tag.start_el(wrapper_tag_name).finish();
    for item in items {
        write(&mut items_tag, "RoleUsageType", item);
    }
    items_tag.finish()
}

pub(crate) fn write(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, item: &RoleUsageType) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    write_tag_with_value(&mut wrapper_tag, "Region", item.region());

    let mut resources_tag = wrapper_tag.start_el("Resources").finish();
    for resource in item.resources() {
        write_tag_with_value(&mut resources_tag, "Resource", Some(resource));
    }
    resources_tag.finish();

    wrapper_tag.finish();
}
