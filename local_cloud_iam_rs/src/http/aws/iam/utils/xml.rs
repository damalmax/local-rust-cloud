use aws_sdk_iam::types::Tag;
use aws_smithy_xml::encode::ScopeWriter;

pub(crate) fn write_tags(parent_tag: &mut ScopeWriter, tags: &[Tag]) {
    local_cloud_xml::write_key_value_tags(
        parent_tag,
        tags,
        |t: &Tag| Some(t.key().to_owned()),
        |t: &Tag| Some(t.value().to_owned()),
    );
}
