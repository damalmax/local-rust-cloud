use aws_sdk_iam::types::VirtualMfaDevice;
use aws_smithy_xml::encode::ScopeWriter;
use data_encoding::{BASE32, BASE64};

use local_cloud_xml::{write_iso8061_datetime_value_tag, write_tag_with_value};

pub(crate) fn write_slice(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, items: &[VirtualMfaDevice]) {
    let mut items_tag = parent_tag.start_el(wrapper_tag_name).finish();
    for item in items {
        write(&mut items_tag, "member", item);
    }
    items_tag.finish()
}

pub(crate) fn write(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, device: &VirtualMfaDevice) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    write_iso8061_datetime_value_tag(&mut wrapper_tag, "EnableDate", device.enable_date());
    write_tag_with_value(&mut wrapper_tag, "SerialNumber", Some(device.serial_number()));
    write_tag_with_value(
        &mut wrapper_tag,
        "Base32StringSeed",
        device.base32_string_seed().map(|v| BASE32.encode(v.as_ref())),
    );
    write_tag_with_value(&mut wrapper_tag, "QRCodePNG", device.qr_code_png().map(|v| BASE64.encode(v.as_ref())));
    if let Some(user) = device.user() {
        super::users::write(&mut wrapper_tag, "User", user);
    }
    super::tags::write_slice(&mut wrapper_tag, device.tags());
    wrapper_tag.finish();
}
