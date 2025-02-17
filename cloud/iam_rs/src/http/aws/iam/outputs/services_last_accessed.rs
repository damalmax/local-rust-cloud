use aws_sdk_iam::types::ServiceLastAccessed;
use aws_smithy_xml::encode::ScopeWriter;

use xml::{write_iso8061_datetime_value_tag, write_tag_with_value};

pub(crate) fn write_slice(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, items: &[ServiceLastAccessed]) {
    let mut items_tag = parent_tag.start_el(wrapper_tag_name).finish();
    for item in items {
        write(&mut items_tag, "member", item);
    }
    items_tag.finish()
}

pub(crate) fn write(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, item: &ServiceLastAccessed) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    write_tag_with_value(&mut wrapper_tag, "ServiceName", Some(item.service_name()));
    write_tag_with_value(&mut wrapper_tag, "ServiceNamespace", Some(item.service_namespace()));
    write_iso8061_datetime_value_tag(&mut wrapper_tag, "LastAuthenticated", item.last_authenticated());
    write_tag_with_value(&mut wrapper_tag, "LastAuthenticatedEntity", item.last_authenticated_entity());
    write_tag_with_value(&mut wrapper_tag, "LastAuthenticatedRegion", item.last_authenticated_region());
    write_tag_with_value(
        &mut wrapper_tag,
        "TotalAuthenticatedEntities",
        item.total_authenticated_entities().map(|v| v.to_string()),
    );
    super::tracked_actions_last_accessed::write_slice(
        &mut wrapper_tag,
        "TrackedActionsLastAccessed",
        item.tracked_actions_last_accessed(),
    );
    wrapper_tag.finish();
}
