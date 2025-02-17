use aws_sdk_iam::types::MfaDevice;
use aws_smithy_xml::encode::ScopeWriter;

use xml::{write_iso8061_datetime_value_tag, write_tag_with_value};

pub(crate) fn write_slice(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, items: &[MfaDevice]) {
    let mut items_tag = parent_tag.start_el(wrapper_tag_name).finish();
    for item in items {
        write(&mut items_tag, "member", item);
    }
    items_tag.finish()
}

pub(crate) fn write(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, item: &MfaDevice) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    write_tag_with_value(&mut wrapper_tag, "UserName", Some(item.user_name()));
    write_tag_with_value(&mut wrapper_tag, "SerialNumber", Some(item.serial_number()));
    write_iso8061_datetime_value_tag(&mut wrapper_tag, "EnableDate", Some(item.enable_date()));
    wrapper_tag.finish();
}
