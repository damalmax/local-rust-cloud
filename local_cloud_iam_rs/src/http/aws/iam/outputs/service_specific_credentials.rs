use aws_sdk_iam::types::ServiceSpecificCredential;
use aws_smithy_xml::encode::ScopeWriter;

use local_cloud_xml::{write_iso8061_datetime_value_tag, write_tag_with_value};

pub(crate) fn write(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, credential: &ServiceSpecificCredential) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    write_tag_with_value(&mut wrapper_tag, "ServicePassword", Some(credential.service_password()));
    write_tag_with_value(&mut wrapper_tag, "ServiceName", Some(credential.service_name()));
    write_tag_with_value(&mut wrapper_tag, "UserName", Some(credential.user_name()));
    write_tag_with_value(&mut wrapper_tag, "ServiceUserName", Some(credential.service_user_name()));
    write_tag_with_value(
        &mut wrapper_tag,
        "ServiceSpecificCredentialId",
        Some(credential.service_specific_credential_id()),
    );
    write_iso8061_datetime_value_tag(&mut wrapper_tag, "CreateDate", Some(credential.create_date()));
    write_tag_with_value(&mut wrapper_tag, "Status", Some(credential.status().as_str()));
    wrapper_tag.finish();
}
