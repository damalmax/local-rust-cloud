use aws_sdk_iam::types::ServiceSpecificCredentialMetadata;
use aws_smithy_xml::encode::ScopeWriter;

use local_cloud_xml::{write_iso8061_datetime_value_tag, write_tag_with_value};

pub(crate) fn write_slice(
    parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, items: &[ServiceSpecificCredentialMetadata],
) {
    let mut items_tag = parent_tag.start_el(wrapper_tag_name).finish();
    for item in items {
        write(&mut items_tag, "member", item);
    }
    items_tag.finish()
}

pub(crate) fn write(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, item: &ServiceSpecificCredentialMetadata) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    write_tag_with_value(&mut wrapper_tag, "UserName", Some(item.user_name()));
    write_tag_with_value(&mut wrapper_tag, "Status", Some(item.status().as_str()));
    write_tag_with_value(&mut wrapper_tag, "ServiceUserName", Some(item.service_user_name()));
    write_iso8061_datetime_value_tag(&mut wrapper_tag, "CreateDate", Some(item.create_date()));
    write_tag_with_value(&mut wrapper_tag, "ServiceSpecificCredentialId", Some(item.service_specific_credential_id()));
    write_tag_with_value(&mut wrapper_tag, "ServiceName", Some(item.service_name()));
    wrapper_tag.finish();
}
