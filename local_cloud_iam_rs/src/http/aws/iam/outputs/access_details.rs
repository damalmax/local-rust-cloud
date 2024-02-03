use aws_sdk_iam::types::AccessDetail;
use aws_smithy_xml::encode::ScopeWriter;

use local_cloud_xml::{write_iso8061_datetime_value_tag, write_tag_with_value};

pub(crate) fn write_slice(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, details: &[AccessDetail]) {
    let mut details_tag = parent_tag.start_el(wrapper_tag_name).finish();
    for detail in details {
        write(&mut details_tag, "member", detail);
    }
    details_tag.finish()
}

pub(crate) fn write(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, detail: &AccessDetail) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    write_tag_with_value(&mut wrapper_tag, "EntityPath", detail.entity_path());
    write_iso8061_datetime_value_tag(&mut wrapper_tag, "LastAuthenticatedTime", detail.last_authenticated_time());
    write_tag_with_value(&mut wrapper_tag, "Region", detail.region());
    write_tag_with_value(&mut wrapper_tag, "ServiceName", Some(detail.service_name()));
    write_tag_with_value(&mut wrapper_tag, "ServiceNamespace", Some(detail.service_namespace()));
    write_tag_with_value(
        &mut wrapper_tag,
        "TotalAuthenticatedEntities",
        detail.total_authenticated_entities().map(|v| v.to_string()),
    );
    wrapper_tag.finish();
}
