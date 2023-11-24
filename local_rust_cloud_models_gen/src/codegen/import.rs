use super::visibility::Visibility;
use std::fmt::Write;

#[allow(dead_code)]
#[derive(Debug)]
pub struct UseWriter<'a, 'b> {
    doc: Option<&'a mut String>,
    name: &'b str,
}

impl<'a, 'b> UseWriter<'a, 'b> {
    pub fn new(doc: &'a mut String, name: &'b str) -> UseWriter<'a, 'b> {
        write!(doc, "use {}", name).unwrap();
        UseWriter { doc: Some(doc), name }
    }

    fn write_end(doc: &mut String) {
        write!(doc, ";\n").unwrap();
    }

    pub fn finish(mut self) {
        let doc = self.doc.take().unwrap();
        Self::write_end(doc);
    }
}

impl Drop for UseWriter<'_, '_> {
    fn drop(&mut self) {
        if let Some(doc) = self.doc.take() {
            Self::write_end(doc);
        }
    }
}
