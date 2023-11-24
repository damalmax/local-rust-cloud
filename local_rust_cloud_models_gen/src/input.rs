use std::fs;
use std::{collections::HashMap, io::Error};

use crate::codegen::encode::{CodeWriter, Visibility};
use crate::smithy::shape::shape::Shape;
use crate::utils;

#[derive(Debug)]
pub struct InputWriter {}

impl InputWriter {
    pub fn new() -> InputWriter {
        InputWriter {}
    }

    pub fn write_input_for_operation(
        &self, target: impl Into<String>, operation_name: &str, shapes: &HashMap<String, Shape>,
    ) -> Result<(), Error> {
        match shapes.get(operation_name).unwrap() {
            Shape::Operation(operation) => {
                let operation_name = utils::nice_name(operation_name);

                let mut source_code = String::new();
                let mut writer = CodeWriter::new(&mut source_code);
                writer.new_mod(Visibility::Public, &operation_name);

                let target_module: String = target.into();

                let operation_dir = target_module + "/" + utils::nice_name(operation_name).as_ref();
                fs::create_dir_all(operation_dir)?;
            }
            _ => return Result::Err(Error::new(std::io::ErrorKind::InvalidData, "unable to find operation")),
        }
        return Result::Ok(());
    }
}
