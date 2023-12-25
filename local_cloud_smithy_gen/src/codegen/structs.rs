use std::fmt::Write;

use crate::codegen::struct_field::StructFieldWriter;

use super::visibility::Visibility;

#[derive(Debug)]
pub struct StructWriter<'a, 'b> {
    doc: Option<&'a mut String>,
    name: &'b str,
    visibility: Visibility,
    documentation: Option<Vec<String>>,
    custom_attributes: Vec<String>,
    derive: Vec<String>,
    fields: Vec<StructFieldWriter>,
}

impl<'a, 'b> StructWriter<'a, 'b> {
    pub fn builder(doc: &'a mut String, name: &'b str) -> StructWriterBuilder<'a, 'b> {
        StructWriterBuilder {
            doc,
            name,
            visibility: Option::None,
            documentation: Option::None,
            custom_attributes: vec![],
            derive: vec![],
            fields: vec![],
        }
    }

    fn write_docs(&mut self, doc: &mut String) {
        if let Some(documentation) = self.documentation.take() {
            for line in &documentation {
                writeln!(doc, "/// {}", line).unwrap();
            }
        } else {
            writeln!(doc, "#[allow(missing_docs)] // documentation missing in model").unwrap();
        }
    }

    fn write_custom_attributes(&mut self, doc: &mut String) {
        if self.custom_attributes.len() > 0 {
            for item in &self.custom_attributes {
                writeln!(doc, "{}", item).unwrap();
            }
        }
    }

    fn write_derive(&mut self, doc: &mut String) {
        if self.derive.len() > 0 {
            write!(doc, "#[derive(").unwrap();
            let mut first = true;
            for item in &self.derive {
                if first {
                    write!(doc, "{}", item).unwrap();
                    first = false;
                } else {
                    write!(doc, ", {}", item).unwrap();
                }
            }
            writeln!(doc, ")]").unwrap();
        }
    }

    fn write_declaration(&mut self, doc: &mut String) {
        let mut doc = doc;
        match self.visibility {
            Visibility::Private => write!(doc, "struct {} {{\n", self.name).unwrap(),
            Visibility::Public => write!(doc, "pub struct {} {{\n", self.name).unwrap(),
            Visibility::PublicCrate => write!(doc, "pub(crate) struct {} {{\n", self.name).unwrap(),
        };
    }

    fn write_fields(&self, doc: &mut String) {
        for field in &self.fields {
            field.write_to(doc);
        }
    }

    fn write_end(doc: &mut String) {
        write!(doc, "}}\n\n").unwrap();
    }

    pub fn finish(mut self) {
        let doc = self.doc.take().unwrap();

        self.write_docs(doc);
        self.write_custom_attributes(doc);
        self.write_derive(doc);
        self.write_declaration(doc);
        self.write_fields(doc);

        Self::write_end(doc);
    }
}

impl Drop for StructWriter<'_, '_> {
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
    fields: Vec<StructFieldWriter>,
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

    pub fn with_fields(mut self, fields: Vec<StructFieldWriter>) -> StructWriterBuilder<'a, 'b> {
        for field in fields {
            self.fields.push(field);
        }
        self.fields.sort_by(|f1, f2| f1.name.cmp(&f2.name));
        self
    }

    pub fn build(self) -> StructWriter<'a, 'b> {
        StructWriter {
            doc: Option::Some(self.doc),
            name: self.name,
            visibility: self.visibility.unwrap_or(Visibility::Public),
            documentation: self.documentation,
            custom_attributes: self.custom_attributes,
            derive: self.derive,
            fields: self.fields,
        }
    }
}
