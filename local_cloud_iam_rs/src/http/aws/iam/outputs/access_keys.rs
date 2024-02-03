use aws_sdk_iam::types::AccessKey;
use aws_smithy_xml::encode::ScopeWriter;

use local_cloud_xml::{write_iso8061_datetime_value_tag, write_tag_with_value};

pub(crate) fn write(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, access_key: &AccessKey) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    write_tag_with_value(&mut wrapper_tag, "UserName", Some(access_key.user_name()));
    write_tag_with_value(&mut wrapper_tag, "AccessKeyId", Some(access_key.access_key_id()));
    write_tag_with_value(&mut wrapper_tag, "Status", Some(access_key.status().as_str()));
    write_tag_with_value(&mut wrapper_tag, "SecretAccessKey", Some(access_key.secret_access_key()));
    write_iso8061_datetime_value_tag(&mut wrapper_tag, "CreateDate", access_key.create_date());
    wrapper_tag.finish();
}
