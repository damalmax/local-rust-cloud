use super::visibility::{self, Visibility};
use std::fmt::Write;

#[derive(Debug)]
pub struct StructWriter<'a> {
    doc: Option<&'a mut String>,
}

impl<'a, 'b> StructWriter<'a> {
    pub fn builder(doc: &'a mut String, name: &'b str) -> StructWriterBuilder<'a, 'b> {
        StructWriterBuilder {
            doc,
            name,
            visibility: Option::None,
            documentation: Option::None,
            custom_attributes: vec![],
            derive: vec![],
        }
    }

    fn write_end(doc: &mut String) {
        write!(doc, "}}\n\n").unwrap();
    }

    pub fn finish(mut self) {
        let doc = self.doc.take().unwrap();
        Self::write_end(doc);
    }
}

impl Drop for StructWriter<'_> {
    fn drop(&mut self) {
        if let Some(doc) = self.doc.take() {
            Self::write_end(doc);
        }
    }
}

#[derive(Debug)]
pub struct StructWriterBuilder<'a, 'b> {
    doc: &'a mut String,
    name: &'b str,
    visibility: Option<Visibility>,
    documentation: Option<Vec<String>>,
    custom_attributes: Vec<String>,
    derive: Vec<String>,
}

impl<'a, 'b> StructWriterBuilder<'a, 'b> {
    pub fn with_docs(mut self, docs: Option<String>) -> StructWriterBuilder<'a, 'b> {
        if let Some(docs) = docs {
            let lines: Vec<String> = docs.split("\n").map(|l| l.to_string()).collect();
            self.documentation = Some(lines);
        }
        self
    }

    pub fn with_visibility(mut self, visibility: Visibility) -> StructWriterBuilder<'a, 'b> {
        self.visibility = Option::Some(visibility);
        self
    }

    pub fn with_derive(mut self, derive: Vec<impl Into<String>>) -> StructWriterBuilder<'a, 'b> {
        for item in derive {
            self.derive.push(item.into());
        }
        self
    }

    pub fn with_custom_attributes(mut self, attributes: Vec<impl Into<String>>) -> StructWriterBuilder<'a, 'b> {
        for item in attributes {
            self.custom_attributes.push(item.into());
        }
        self
    }

    fn write_docs(&mut self) {
        if let Some(documentation) = self.documentation.take() {
            for line in &documentation {
                writeln!(self.doc, "/// {}", line).unwrap();
            }
        } else {
            writeln!(self.doc, "#[allow(missing_docs)] // documentation missing in model").unwrap();
        }
    }

    fn write_custom_attributes(&mut self) {
        if self.custom_attributes.len() > 0 {
            for item in &self.custom_attributes {
                write!(self.doc, "{}\n", item).unwrap();
            }
        }
    }

    fn write_derive(&mut self) {
        if self.derive.len() > 0 {
            write!(self.doc, "#[derive(").unwrap();
            let mut first = true;
            for item in &self.derive {
                if first {
                    write!(self.doc, "{}", item).unwrap();
                    first = false;
                } else {
                    write!(self.doc, ", {}", item).unwrap();
                }
            }
            write!(self.doc, ")]\n").unwrap();
        }
    }

    fn write_declaration(&mut self) {
        match self.visibility.as_ref().unwrap_or(&Visibility::Private) {
            Visibility::Private => write!(self.doc, "struct {} {{\n", self.name).unwrap(),
            Visibility::Public => write!(self.doc, "pub struct {} {{\n", self.name).unwrap(),
            Visibility::PublicCrate => write!(self.doc, "pub(crate) struct {} {{\n", self.name).unwrap(),
        };
    }

    pub fn build(mut self) -> StructWriter<'a> {
        self.write_docs();
        self.write_custom_attributes();
        self.write_derive();
        self.write_declaration();

        StructWriter {
            doc: Option::Some(self.doc),
        }
    }
}
