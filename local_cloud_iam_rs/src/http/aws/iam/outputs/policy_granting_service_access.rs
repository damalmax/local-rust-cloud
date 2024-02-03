use aws_sdk_iam::types::PolicyGrantingServiceAccess;
use aws_smithy_xml::encode::ScopeWriter;

use local_cloud_xml::write_tag_with_value;

pub(crate) fn write_slice(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, items: &[PolicyGrantingServiceAccess]) {
    let mut items_tag = parent_tag.start_el(wrapper_tag_name).finish();
    for item in items {
        write(&mut items_tag, "member", item);
    }
    items_tag.finish()
}

pub(crate) fn write(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, item: &PolicyGrantingServiceAccess) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    write_tag_with_value(&mut wrapper_tag, "PolicyName", Some(item.policy_name()));
    write_tag_with_value(&mut wrapper_tag, "PolicyType", Some(item.policy_type().as_str()));
    write_tag_with_value(&mut wrapper_tag, "PolicyArn", item.policy_arn());
    write_tag_with_value(&mut wrapper_tag, "EntityType", item.entity_type().map(|v| v.as_str()));
    write_tag_with_value(&mut wrapper_tag, "EntityName", item.entity_name());
    wrapper_tag.finish();
}
