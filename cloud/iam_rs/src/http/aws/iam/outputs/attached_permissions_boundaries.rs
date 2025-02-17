use aws_sdk_iam::types::AttachedPermissionsBoundary;
use aws_smithy_xml::encode::ScopeWriter;

use xml::write_tag_with_value;

pub(crate) fn write_slice(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, items: &[AttachedPermissionsBoundary]) {
    let mut items_tag = parent_tag.start_el(wrapper_tag_name).finish();
    for item in items {
        write(&mut items_tag, "member", item);
    }
    items_tag.finish()
}

pub(crate) fn write(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, item: &AttachedPermissionsBoundary) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    write_tag_with_value(&mut wrapper_tag, "PermissionsBoundaryArn", item.permissions_boundary_arn());
    write_tag_with_value(
        &mut wrapper_tag,
        "PermissionsBoundaryType",
        item.permissions_boundary_type().map(|v| v.as_str()),
    );
    wrapper_tag.finish();
}
