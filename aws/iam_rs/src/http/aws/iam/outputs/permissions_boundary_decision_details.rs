use aws_sdk_iam::types::PermissionsBoundaryDecisionDetail;
use aws_smithy_xml::encode::ScopeWriter;

use xml::write_tag_with_value;

pub(crate) fn write_slice(
    parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, items: &[PermissionsBoundaryDecisionDetail],
) {
    let mut items_tag = parent_tag.start_el(wrapper_tag_name).finish();
    for item in items {
        write(&mut items_tag, "member", item);
    }
    items_tag.finish()
}

pub(crate) fn write(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, item: &PermissionsBoundaryDecisionDetail) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    write_tag_with_value(
        &mut wrapper_tag,
        "AllowedByPermissionsBoundary",
        Some(item.allowed_by_permissions_boundary().to_string()),
    );
    wrapper_tag.finish();
}
