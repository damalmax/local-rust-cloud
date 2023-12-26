use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{Error, ErrorKind, Write};
use std::path::Path;

use crate::codegen::enum_field::EnumFieldWriter;
use crate::codegen::struct_field::StructFieldWriter;
use crate::codegen::visibility::Visibility;
use crate::codegen::writer::CodeWriter;
use crate::naming::Naming;
use crate::smithy::shape::enum_shape::EnumShape;
use crate::smithy::shape::operation::OperationShape;
use crate::smithy::shape::shape::Shape;
use crate::smithy::shape::structure::StructureShape;

#[derive(Debug)]
struct Shapes<'a> {
    shapes: &'a HashMap<String, Shape>,
}

impl<'a> Shapes<'a> {
    fn new(shapes: &'a HashMap<String, Shape>) -> Shapes {
        Shapes { shapes }
    }

    fn get_shape(&self, id: &str) -> Option<&Shape> {
        self.shapes.get(id)
    }
}

#[derive(Debug)]
pub struct SmithyCodegen<'a, 'b> {
    base_dir: &'b str,
    base_package: &'b str,
    shapes: &'a HashMap<String, Shape>,
}

impl<'a, 'b> SmithyCodegen<'a, 'b> {
    pub fn new(base_dir: &'b str, base_package: &'b str, shapes: &'a HashMap<String, Shape>) -> SmithyCodegen<'a, 'b> {
        fs::create_dir_all(base_dir).expect(&format!("failed to create base directory '{base_dir}' for generated files"));
        SmithyCodegen {
            base_dir,
            base_package,
            shapes,
        }
    }

    pub fn generate_types(&self) -> Result<(), Error> {
        let types_base_dir = &format!("{}/types", self.base_dir);
        fs::create_dir_all(types_base_dir).expect(&format!("failed to create base directory '{types_base_dir}' for types"));

        let mut types = vec![];
        for (key, shape) in self.shapes {
            if key.is_smithy_unit() {
                // the type means Void per Smithy documentation. Ignore if defined.
                continue;
            }

            let filename = key.to_smithy_filename();
            let path = &format!("{types_base_dir}/{}.rs", &filename);
            match shape {
                Shape::Structure(structure_shape) => {
                    with_code_writer(Path::new(path), &mut |writer: &mut CodeWriter| {
                        self.generate_structure_for_shape(writer, key, &key.to_smithy_struct_name(), structure_shape)?;
                        Result::Ok(())
                    })?;
                    // structure is successfully generated. Save it for 'mod.rs' generation.
                    types.push(filename.clone());
                }
                Shape::Enum(enum_shape) => {
                    with_code_writer(Path::new(path), &mut |writer: &mut CodeWriter| {
                        self.generate_enum_for_shape(writer, &key.to_smithy_struct_name(), enum_shape)?;
                        Result::Ok(())
                    })?;
                    // structure is successfully generated. Save it for 'mod.rs' generation.
                    types.push(filename.clone());
                }
                _ => {}
            };
        }
        with_code_writer(Path::new(&format!("{types_base_dir}/mod.rs")), &mut |writer: &mut CodeWriter| {
            types.sort();
            for type_name in &types {
                writer.new_mod(Visibility::Public, &type_name).finish();
            }
            Result::Ok(())
        })?;
        Result::Ok(())
    }

    pub fn generate_operations(&self) -> Result<(), Error> {
        let operations_base_dir = &format!("{}/operations", self.base_dir);
        fs::create_dir_all(operations_base_dir).expect(&format!("failed to create base directory '{operations_base_dir}' for operations"));

        let mut operations = vec![];
        for (key, shape) in self.shapes {
            if key != "com.amazonaws.iam#CreatePolicy" {
                continue;
            }
            match shape {
                Shape::Operation(operation_shape) => {
                    let filename = key.to_smithy_filename();
                    let operation_base_dir = &format!("{operations_base_dir}/{}", &filename);
                    fs::create_dir_all(operation_base_dir)
                        .expect(&format!("failed to create base directory '{operation_base_dir}' for operation"));

                    let path = &format!("{operation_base_dir}/operation.rs");
                    with_code_writer(Path::new(path), &mut |writer: &mut CodeWriter| {
                        self.generate_operation_for_shape(writer, &key.to_smithy_struct_name(), operation_shape)?;
                        Result::Ok(())
                    })?;
                    // Operation code is successfully generated. Save it for 'mod.rs' generation.
                    operations.push(filename.clone());

                    // Create 'mod.rs' file for files inside '{operation}' folder.
                    with_code_writer(Path::new(&format!("{operation_base_dir}/mod.rs")), &mut |writer: &mut CodeWriter| {
                        writer.new_mod(Visibility::Public, "operation").finish();
                        Result::Ok(())
                    })?;
                }
                _ => {}
            };
        }
        with_code_writer(Path::new(&format!("{operations_base_dir}/mod.rs")), &mut |writer: &mut CodeWriter| {
            for operation_name in &operations {
                writer.new_mod(Visibility::Public, &operation_name).finish();
            }
            Result::Ok(())
        })
    }

    fn generate_operation_for_shape(&self, writer: &mut CodeWriter, name: &str, structure_shape: &OperationShape) -> Result<(), Error> {
        Result::Ok(())
    }

    fn generate_enum_for_shape(&self, writer: &mut CodeWriter, name: &str, enum_shape: &EnumShape) -> Result<(), Error> {
        writer
            .new_enum_builder(name)
            .with_docs(enum_shape.documentation())
            .with_visibility(Visibility::Public)
            .with_derive(vec!["Clone", "PartialEq", "Debug"])
            .with_fields(self.generate_enum_fields_for_shape(enum_shape)?)
            .build()
            .finish();
        Result::Ok(())
    }

    fn generate_enum_fields_for_shape(&self, enum_shape: &EnumShape) -> Result<Vec<EnumFieldWriter>, Error> {
        let mut fields = vec![];

        for (key, member) in &enum_shape.members {
            let rename_macro = format!("#[serde(rename = \"{}\")]", member.traits.enum_value);
            let field_name = key.to_smithy_struct_name();
            let writer = EnumFieldWriter::builder(&field_name)
                .with_custom_attributes(vec![rename_macro])
                .with_docs(member.documentation())
                .build();
            fields.push(writer);
        }
        Result::Ok(fields)
    }

    fn generate_structure_for_shape(
        &self, writer: &mut CodeWriter, target: &str, name: &str, structure_shape: &StructureShape,
    ) -> Result<(), Error> {
        writer
            .new_struct_builder(name)
            .with_docs(structure_shape.documentation())
            .with_visibility(Visibility::Public)
            .with_custom_attributes(vec!["#[non_exhaustive]"])
            .with_derive(vec!["Clone", "PartialEq", "Debug", "derive_builder::Builder"])
            .with_fields(self.generate_structure_fields_for_shape(target, structure_shape)?)
            .build()
            .finish();
        Result::Ok(())
    }

    fn generate_structure_fields_for_shape(&self, target: &str, structure_shape: &StructureShape) -> Result<Vec<StructFieldWriter>, Error> {
        let mut fields = vec![];

        if self.is_list_member(target) {
            let writer = StructFieldWriter::builder("list_member_idx", "i32")
                .set_optional(true)
                .with_visibility(Visibility::Public)
                .build();
            fields.push(writer);
        }

        for (key, member) in &structure_shape.members {
            let field_name = key.to_smithy_field_name();
            let writer = StructFieldWriter::builder(&field_name, self.get_type_for_structure_field(&member.target)?)
                .set_optional(member.is_optional())
                .with_docs(member.documentation())
                .with_visibility(Visibility::Public)
                .build();
            fields.push(writer);
        }
        Result::Ok(fields)
    }

    fn is_list_member(&self, target: &str) -> bool {
        for (_, shape) in self.shapes {
            match shape {
                Shape::List(list_shape) => {
                    if list_shape.member.target == target {
                        return true;
                    }
                }
                _ => {}
            }
        }
        return false;
    }

    fn get_type_for_structure_field(&self, target: &str) -> Result<String, Error> {
        if target == "smithy.api#Unit" {
            return Result::Ok(String::from("()"));
        }

        let shape = &self.shapes.get(target);
        match shape {
            None => Result::Err(Error::new(ErrorKind::InvalidData, format!("unable to find definition for shape type '{target}'"))),
            Some(shape) => match shape {
                Shape::Structure(_) => {
                    Result::Ok(format!("{}::types::{}::{}", self.base_package, target.to_smithy_filename(), target.to_smithy_struct_name()))
                }
                Shape::Enum(_) => {
                    Result::Ok(format!("{}::types::{}::{}", self.base_package, target.to_smithy_filename(), target.to_smithy_struct_name()))
                }
                Shape::List(list) => {
                    let list_member_target = &list.member.target;
                    Result::Ok(format!("Vec<{}>", self.get_type_for_structure_field(list_member_target)?.to_string()))
                }
                Shape::String(_) => Result::Ok(String::from("String")),
                Shape::Blob(_) => Result::Ok(String::from("Vec<u8>")),
                Shape::Map(map) => {
                    let key_target = &map.key.target;
                    let value_target = &map.value.target;
                    Result::Ok(format!(
                        "std::collections::HashMap<{}, {}>",
                        self.get_type_for_structure_field(key_target)?,
                        self.get_type_for_structure_field(value_target)?
                    ))
                }
                Shape::Integer(_) => Result::Ok(String::from("i32")),
                Shape::Double(_) => Result::Ok(String::from("f64")),
                Shape::Boolean(_) => Result::Ok(String::from("bool")),
                Shape::Timestamp => Result::Ok(String::from("aws_smithy_types::DateTime")),
                Shape::Long(_) => Result::Ok(String::from("i64")),
                Shape::Float(_) => Result::Ok(String::from("f32")),
                Shape::Union(_) => Result::Ok(format!("{}Union", self.base_package)),
                _ => {
                    Result::Err(Error::new(ErrorKind::InvalidData, format!("unexpected field type '{target}'. please check Smithy source")))
                }
            },
        }
    }
}

fn with_code_writer(target: &Path, generator: &mut dyn FnMut(&mut CodeWriter) -> Result<(), Error>) -> Result<(), Error> {
    let mut content = String::new();
    let mut writer = CodeWriter::new(&mut content);

    generator(&mut writer)?;

    File::create(target)
        .expect(&format!("failed to open file '{}' for edit", target.to_str().unwrap()))
        .write_all(&content.as_bytes())
        .expect(&format!("failed to write generated code to file '{}' structures", target.to_str().unwrap()));
    Result::Ok(())
}
