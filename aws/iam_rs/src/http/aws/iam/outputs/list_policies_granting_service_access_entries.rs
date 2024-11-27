use aws_sdk_iam::types::ListPoliciesGrantingServiceAccessEntry;
use aws_smithy_xml::encode::ScopeWriter;

use xml::write_tag_with_value;

pub(crate) fn write_slice(
    parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, items: &[ListPoliciesGrantingServiceAccessEntry],
) {
    let mut items_tag = parent_tag.start_el(wrapper_tag_name).finish();
    for item in items {
        write(&mut items_tag, "member", item);
    }
    items_tag.finish()
}

pub(crate) fn write(
    parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, item: &ListPoliciesGrantingServiceAccessEntry,
) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    write_tag_with_value(&mut wrapper_tag, "ServiceNamespace", item.service_namespace());
    super::policy_granting_service_access::write_slice(&mut wrapper_tag, "Policies", item.policies());
    wrapper_tag.finish();
}
