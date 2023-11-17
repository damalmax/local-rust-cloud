use aws_smithy_types::date_time::Format;
use aws_smithy_xml::encode::ScopeWriter;

///Adds a new sub-tag to the parent tag with the given name in case if value is defined.
pub fn write_tag_with_value(parent_tag: &mut ScopeWriter, child_tag_name: &str, value: Option<impl Into<String>>) {
    if value.is_some() {
        let mut child_tag = parent_tag.start_el(child_tag_name).finish();
        child_tag.data(value.unwrap().into().as_str());
        child_tag.finish();
    }
}

///Adds a new sub-tag to the parent tag with the given name in case if value is defined.
pub fn write_tag_with_date_value(parent_tag: &mut ScopeWriter, child_tag_name: &str, value: Option<&aws_smithy_types::DateTime>) {
    if value.is_some() {
        let mut child_tag = parent_tag.start_el(child_tag_name).finish();
        child_tag.data(
            value
                .map(|date_time| date_time.fmt(Format::DateTime).expect("Failed to format date"))
                .expect("Failed to format date")
                .as_str(),
        );
        child_tag.finish();
    }
}

pub fn write_key_value_tags<T>(
    parent_tag: &mut ScopeWriter, group_name: &str, tags: Option<&[T]>, key_mapper: fn(&T) -> Option<String>,
    value_mapper: fn(&T) -> Option<String>,
) {
    if tags.is_some() {
        let mut tags_tag = parent_tag.start_el(group_name).finish();
        let tags = tags.unwrap();
        for tag in tags {
            let mut tag_tag = tags_tag.start_el("Tag").finish();
            write_tag_with_value(&mut tag_tag, "Key", key_mapper(&tag));
            write_tag_with_value(&mut tag_tag, "Value", value_mapper(&tag));
            tag_tag.finish();
        }
        tags_tag.finish();
    }
}

#[cfg(test)]
mod test {
    use aws_smithy_protocol_test::{assert_ok, validate_body, MediaType};
    use aws_smithy_xml::encode::XmlWriter;

    #[test]
    fn test_write_tag_with_value() {
        let mut out = String::new();
        let mut doc_writer = XmlWriter::new(&mut out);
        let mut start_el = doc_writer.start_el("Root").write_ns("https://example.com", None);
        start_el.write_attribute("key1", "att1");
        let mut tag = start_el.finish();
        crate::write_tag_with_value(&mut tag, "inner", Option::Some("value1"));
        crate::write_tag_with_value(&mut tag, "inner", Option::Some("value2"));
        tag.finish();

        assert_ok(validate_body(
            out,
            r#"<Root key1="att1" xmlns="https://example.com">
                    <inner>value1</inner>
                    <inner>value2</inner>
                </Root>"#,
            MediaType::Xml,
        ));
    }
}
