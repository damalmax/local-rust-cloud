use convert_case::{Case, Casing};

pub trait Naming<T: AsRef<str>> {
    fn is_smithy_unit(&self) -> bool;

    fn to_smithy_field_name(&self) -> String;

    fn to_smithy_filename(&self) -> String;

    fn to_smithy_struct_name(&self) -> String;
}

impl<T: AsRef<str>> Naming<T> for T
where
    String: PartialEq<T>,
{
    fn is_smithy_unit(&self) -> bool {
        "smithy.api#Unit" == self.as_ref()
    }

    fn to_smithy_filename(&self) -> String {
        NamingHolder::new(self).remove_unusable_parts().to_case(Case::Snake)
    }

    fn to_smithy_field_name(&self) -> String {
        NamingHolder::new(self).remove_unusable_parts().to_case(Case::Snake)
    }

    fn to_smithy_struct_name(&self) -> String {
        NamingHolder::new(self).remove_unusable_parts().to_case(Case::UpperCamel)
    }
}

struct NamingHolder<'a, T: AsRef<str>> {
    inner: &'a T,
}

impl<'a, T: AsRef<str>> NamingHolder<'a, T> {
    fn new(inner: &'a T) -> Self {
        Self { inner }
    }

    fn remove_unusable_parts(&self) -> String {
        let line = self.inner.as_ref();
        if line.contains("#") {
            line.split_at(line.find('#').unwrap() + 1).1.to_string()
        } else {
            line.to_string()
        }
    }
}
