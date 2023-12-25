use std::fmt::Write;

#[derive(Debug)]
pub struct EnumFieldWriter {
    pub(crate) name: String,
    str_value: Option<String>,
    field_type: Option<String>,
    documentation: Option<Vec<String>>,
    custom_attributes: Vec<String>,
}

impl EnumFieldWriter {
    pub fn builder(name: impl Into<String>) -> EnumFieldBuilder {
        EnumFieldBuilder {
            name: name.into(),
            str_value: Option::None,
            field_type: Option::None,
            documentation: Option::None,
            custom_attributes: vec![],
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
            writeln!(doc, "    #[allow(missing_docs)] // documentation missing in the model").unwrap();
        }
    }

    fn write_custom_attributes(&self, doc: &mut String) {
        if self.custom_attributes.len() > 0 {
            for item in &self.custom_attributes {
                writeln!(doc, "    {}", item).unwrap();
            }
        }
    }

    fn write_definition(&self, doc: &mut String) {
        match &self.field_type {
            None => write!(doc, "    {}", self.name).unwrap(),
            Some(custom_type) => writeln!(doc, "    {}({})", self.name, custom_type).unwrap(),
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
pub struct EnumFieldBuilder {
    name: String,
    str_value: Option<String>,
    field_type: Option<String>,
    documentation: Option<Vec<String>>,
    custom_attributes: Vec<String>,
}

impl EnumFieldBuilder {
    pub fn with_docs(mut self, docs: Option<String>) -> EnumFieldBuilder {
        if let Some(docs) = docs {
            let lines: Vec<String> = docs.split("\n").map(|l| l.to_string()).collect();
            self.documentation = Some(lines);
        }
        self
    }

    pub fn with_custom_attributes(mut self, attributes: Vec<impl Into<String>>) -> EnumFieldBuilder {
        for item in attributes {
            self.custom_attributes.push(item.into());
        }
        self
    }

    pub fn build(self) -> EnumFieldWriter {
        EnumFieldWriter {
            name: self.name,
            str_value: self.str_value,
            field_type: self.field_type,
            documentation: self.documentation,
            custom_attributes: self.custom_attributes,
        }
    }
}
