use super::visibility::Visibility;
use std::fmt::Write;

#[allow(dead_code)]
#[derive(Debug)]
pub struct ModWriter<'a, 'b> {
    doc: Option<&'a mut String>,
    visibility: Visibility,
    name: &'b str,
}

impl<'a, 'b> ModWriter<'a, 'b> {
    pub fn new(doc: &'a mut String, visibility: Visibility, name: &'b str) -> ModWriter<'a, 'b> {
        let visibility_str = match visibility {
            Visibility::Private => "",
            Visibility::Public => "pub ",
            Visibility::PublicCrate => "pub(crate) ",
        };
        write!(doc, "{}mod {}", visibility_str, name).unwrap();
        ModWriter {
            doc: Some(doc),
            visibility,
            name,
        }
    }

    fn write_end(doc: &mut String) {
        write!(doc, ";\n").unwrap();
    }

    pub fn finish(mut self) {
        let doc = self.doc.take().unwrap();
        Self::write_end(doc);
    }
}

impl Drop for ModWriter<'_, '_> {
    fn drop(&mut self) {
        if let Some(doc) = self.doc.take() {
            Self::write_end(doc);
        }
    }
}
