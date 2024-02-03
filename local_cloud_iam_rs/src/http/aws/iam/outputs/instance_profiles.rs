use aws_sdk_iam::types::InstanceProfile;
use aws_smithy_xml::encode::ScopeWriter;

use local_cloud_xml::{write_iso8061_datetime_value_tag, write_tag_with_value};

pub(crate) fn write_slice(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, profiles: &[InstanceProfile]) {
    let mut profiles_tag = parent_tag.start_el(wrapper_tag_name).finish();
    for profile in profiles {
        write(&mut profiles_tag, "member", profile);
    }
    profiles_tag.finish()
}

pub(crate) fn write(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, profile: &InstanceProfile) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    write_tag_with_value(&mut wrapper_tag, "InstanceProfileName", Some(profile.instance_profile_name()));
    write_tag_with_value(&mut wrapper_tag, "Path", Some(profile.path()));
    super::roles::write_slice(&mut wrapper_tag, profile.roles());
    write_tag_with_value(&mut wrapper_tag, "Arn", Some(profile.arn()));
    write_tag_with_value(&mut wrapper_tag, "InstanceProfileId", Some(profile.instance_profile_id()));
    write_iso8061_datetime_value_tag(&mut wrapper_tag, "CreateDate", Some(profile.create_date()));
    super::tags::write_slice(&mut wrapper_tag, profile.tags());
    wrapper_tag.finish();
}
