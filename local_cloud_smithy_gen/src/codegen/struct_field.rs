use std::fmt::Write;

use crate::codegen::visibility::Visibility;

#[derive(Debug)]
pub struct StructFieldWriter {
    pub(crate) name: String,
    field_type: String,
    visibility: Visibility,
    documentation: Option<Vec<String>>,
    custom_attributes: Vec<String>,
    is_optional: bool,
}

impl StructFieldWriter {
    pub fn builder(name: impl Into<String>, field_type: impl Into<String>) -> StructFieldBuilder {
        StructFieldBuilder {
            name: name.into(),
            field_type: field_type.into(),
            visibility: None,
            documentation: None,
            custom_attributes: vec![],
            is_optional: None,
        }
    }

    fn write_end(&self, doc: &mut String) {
        writeln!(doc, ",").unwrap();
    }

    fn write_docs(&self, doc: &mut String) {
        if let Some(documentation) = &self.documentation {
            for line in documentation {
                writeln!(doc, "    /// {}", line).unwrap();
            }
        } else {
            writeln!(doc, "    #[allow(missing_docs)] // documentation missing in model").unwrap();
        }
    }

    fn write_custom_attributes(&self, doc: &mut String) {
        if self.custom_attributes.len() > 0 {
            for item in &self.custom_attributes {
                write!(doc, "    {}\n", item).unwrap();
            }
        }
    }

    fn write_definition(&self, doc: &mut String) {
        match self.visibility {
            Visibility::Private => write!(doc, "    {}: {}", self.name, self.get_type()).unwrap(),
            Visibility::Public => write!(doc, "    pub {}: {}", self.name, self.get_type()).unwrap(),
            Visibility::PublicCrate => write!(doc, "    pub(crate) {}: {}", self.name, self.get_type()).unwrap(),
        }
    }

    fn get_type(&self) -> String {
        if self.is_optional {
            format!("std::option::Option<{}>", self.field_type)
        } else {
            self.field_type.to_string()
        }
    }

    pub fn write_to(&self, doc: &mut String) {
        self.write_docs(doc);
        self.write_custom_attributes(doc);
        self.write_definition(doc);

        self.write_end(doc);
    }
}

#[derive(Debug)]
pub struct StructFieldBuilder {
    name: String,
    field_type: String,
    visibility: Option<Visibility>,
    documentation: Option<Vec<String>>,
    is_optional: Option<bool>,
    custom_attributes: Vec<String>,
}

impl StructFieldBuilder {
    pub fn with_docs(mut self, docs: Option<String>) -> StructFieldBuilder {
        if let Some(docs) = docs {
            let lines: Vec<String> = docs.split("\n").map(|l| l.to_string()).collect();
            self.documentation = Some(lines);
        }
        self
    }

    pub fn with_visibility(mut self, visibility: Visibility) -> StructFieldBuilder {
        self.visibility = Some(visibility);
        self
    }

    pub fn set_optional(mut self, optional: bool) -> StructFieldBuilder {
        self.is_optional = Some(optional);
        self
    }

    pub fn with_custom_attributes(mut self, attributes: Vec<impl Into<String>>) -> StructFieldBuilder {
        for item in attributes {
            self.custom_attributes.push(item.into());
        }
        self
    }

    pub fn build(self) -> StructFieldWriter {
        StructFieldWriter {
            name: self.name,
            field_type: self.field_type,
            visibility: self.visibility.unwrap_or(Visibility::Private),
            documentation: self.documentation,
            custom_attributes: self.custom_attributes,
            is_optional: self.is_optional.unwrap_or(false),
        }
    }
}
