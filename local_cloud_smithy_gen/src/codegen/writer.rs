use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter, Write};

use crate::codegen::enums::{EnumWriter, EnumWriterBuilder};

use super::module::ModWriter;
use super::structs::{StructWriter, StructWriterBuilder};
use super::visibility::Visibility;

#[non_exhaustive]
#[derive(Debug)]
pub struct CodeWriterError {}

impl Display for CodeWriterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "error writing Rust code")
    }
}

impl StdError for CodeWriterError {}

pub struct CodeWriter<'a> {
    doc: &'a mut String,
}

impl<'a> CodeWriter<'a> {
    pub fn new(doc: &'a mut String) -> Self {
        write!(doc, "// DO NOT EDIT. The code is generated by Local Rust Cloud models gen tool.\n\n").unwrap();
        Self { doc }
    }
}

impl<'a> CodeWriter<'a> {
    pub fn new_mod<'b, 'c>(&'c mut self, visibility: Visibility, name: &'b str) -> ModWriter<'c, 'b> {
        ModWriter::new(self.doc, visibility, name)
    }

    pub fn new_struct_builder<'b, 'c>(&'c mut self, name: &'b str) -> StructWriterBuilder<'c, 'b> {
        StructWriter::builder(self.doc, name)
    }

    pub fn new_enum_builder<'b, 'c>(&'c mut self, name: &'b str) -> EnumWriterBuilder<'c, 'b> {
        EnumWriter::builder(self.doc, name)
    }
}