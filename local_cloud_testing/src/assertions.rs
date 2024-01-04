pub fn assert_not_empty<'a>(value: impl Into<ValueToCheck<'a>>) {
    let value = value.into();
    match value {
        ValueToCheck::Str(s) => {
            assert_not_empty_str(s);
        }
        ValueToCheck::String(s) => {
            assert_not_empty_str(s.as_deref());
        }
    }
}

fn assert_not_empty_str(value: Option<&str>) {
    assert!(value.is_some());
    assert!(value.unwrap().chars().count() > 0);
}

#[derive(Debug)]
pub enum ValueToCheck<'a> {
    Str(Option<&'a str>),
    String(Option<String>),
}

impl<'a> From<Option<&'a str>> for ValueToCheck<'a> {
    fn from(value: Option<&'a str>) -> Self {
        ValueToCheck::Str(value)
    }
}

impl<'a> From<Option<String>> for ValueToCheck<'a> {
    fn from(value: Option<String>) -> Self {
        ValueToCheck::String(value)
    }
}
