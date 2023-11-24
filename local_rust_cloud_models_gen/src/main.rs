use std::{
    collections::HashMap,
    fs::{self, File},
    io::{Error, ErrorKind, Read, Write},
    path::Path,
};

use clap::{Arg, ArgAction, Command};

use crate::{
    codegen::{visibility::Visibility, writer::CodeWriter},
    smithy::{
        shape::{
            operation::{self, OperationInput, OperationOutput, OperationShape},
            shape::Shape,
            structure::StructureShape,
        },
        smithy::Smithy,
    },
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

mod codegen;
mod smithy;
mod utils;

fn main() -> std::io::Result<()> {
    let matches = Command::new("local_rust_cloud_models_gen")
        .about("Generating models for Local Rust Cloud applications")
        .version(VERSION)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Maksim Dadzerkin <maksim.dadzerkin@gmail.com>")
        // Sync subcommand
        //
        // Only a few of its arguments are implemented below.
        .subcommand(
            Command::new("sync")
                .short_flag('S')
                .long_flag("sync")
                .about("Synchronize packages.")
                .arg(
                    Arg::new("source")
                        .short('s')
                        .long("source")
                        .action(ArgAction::Set)
                        .num_args(1)
                        .required(true)
                        .help("path to the API definition file in JSON format"),
                )
                .arg(
                    Arg::new("target")
                        .short('t')
                        .long("target")
                        .action(ArgAction::Set)
                        .num_args(1)
                        .required(true)
                        .help("path to the target folder where models should be placed"),
                )
                .arg(
                    Arg::new("format")
                        .short('f')
                        .long("format")
                        .action(ArgAction::Set)
                        .default_value("xml")
                        .num_args(1)
                        .help("desired format to be supported for generated shape converters. Allowed values: xml"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("sync", sync_matches)) => {
            let source: &str = sync_matches.get_one::<String>("source").expect("source is provided");
            println!("Source path is '{source}'");
            let target: &str = sync_matches.get_one::<String>("target").expect("target is provided");
            println!("Target path is '{target}'");
            let fmt: &str = sync_matches.get_one::<String>("format").expect("format is provided");
            println!("Format is '{fmt}'");

            generate_source_code(source, target, fmt)?;
            return Result::Ok(());
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }

    fn generate_source_code(source: &str, target: &str, fmt: &str) -> Result<(), Error> {
        let mut file = File::open(source)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // read Smithy model
        let smithy: Smithy = serde_json::from_str(&contents).expect("Failed to read Smithy JSON file");
        let smithy_shapes = &smithy.shapes;

        let base_path = target.trim_end_matches('/').to_string();
        let base_bath = &format!("{base_path}/src/aws/operations");
        fs::create_dir_all(base_bath)?;

        let mut mod_list = String::new();
        let mut operations_mod_writer = CodeWriter::new(&mut mod_list);
        for (key, shape) in smithy_shapes {
            match shape {
                Shape::Operation(operation_shape) => {
                    if key != "com.amazonaws.iam#CreatePolicy" {
                        continue;
                    }
                    let operation_name = &utils::operation_name(key);
                    operations_mod_writer.new_mod(Visibility::Public, &operation_name).finish();
                    create_operation_mod_file(base_bath, operation_name)?;

                    let mut shapes = String::new();
                    let mut shapes_writer = CodeWriter::new(&mut shapes);
                    generate_input_shape(&mut shapes_writer, &operation_shape.input, smithy_shapes, fmt)
                        .expect("failed to write input shape");
                    generate_output_shape(&mut shapes_writer, &operation_shape.output, smithy_shapes, fmt)
                        .expect("failed to write output shape");
                    File::create(format!("{base_bath}/{operation_name}/shapes.rs"))
                        .expect("failed to open file for edit")
                        .write_all(&shapes.as_bytes())
                        .expect("failed to write model structures");

                    let operation_file = &format!("{base_bath}/{operation_name}/operation.rs");
                    if !Path::new(operation_file).exists() {
                        let mut operation = String::new();
                        let mut operation_writer = CodeWriter::new(&mut operation);
                        generate_operation(&mut operation_writer, &operation_name, operation_shape, smithy_shapes)
                            .expect("failed to write operation function");
                        File::create(operation_file)
                            .expect("failed to open file for edit")
                            .write_all(&operation.as_bytes())
                            .expect("failed to write model structures");
                    }
                    // break;
                }
                _ => continue,
            }
        }
        File::create(format!("{base_bath}/mod.rs"))?.write_all(&mod_list.as_bytes())?;
        Result::Ok(())
    }

    fn create_operation_mod_file(base_path: &str, operation_name: &str) -> Result<(), Error> {
        // create folder for operation
        fs::create_dir_all(format!("{base_path}/{operation_name}"))?;

        let mut operation_mod_list = String::new();
        let mut operation_mod_writer = CodeWriter::new(&mut operation_mod_list);
        operation_mod_writer.new_mod(Visibility::Public, "shapes").finish();
        operation_mod_writer.new_mod(Visibility::Public, "operation").finish();
        File::create(format!("{base_path}/{operation_name}/mod.rs"))
            .expect("failed to write operation mod.rs file")
            .write_all(&operation_mod_list.as_bytes())?;

        Result::Ok(())
    }

    fn generate_input_shape(
        writer: &mut CodeWriter, input: &OperationInput, shapes: &HashMap<String, Shape>, fmt: &str,
    ) -> Result<(), Error> {
        let target = &input.target;
        let name = &utils::struct_name(target);
        generate_structure_for_shape(writer, target, name, shapes)?;
        Result::Ok(())
    }

    fn generate_output_shape(
        writer: &mut CodeWriter, output: &OperationOutput, shapes: &HashMap<String, Shape>, fmt: &str,
    ) -> Result<(), Error> {
        let target = &output.target;
        let name = &utils::struct_name(target);
        generate_structure_for_shape(writer, target, name, shapes)?;
        Result::Ok(())
    }

    fn generate_structure_for_shape(
        writer: &mut CodeWriter, target: &str, name: &str, shapes: &HashMap<String, Shape>,
    ) -> Result<(), Error> {
        if target == "smithy.api#Unit" {
            return Result::Ok(());
        }
        println!("processing shape {target}");
        let target_shape = shapes.get(target);
        match target_shape {
            Some(shape) => match shape {
                Shape::Structure(structure_shape) => {
                    writer
                        .new_struct_builder(name)
                        .with_docs(structure_shape.documentation())
                        .with_visibility(Visibility::Public)
                        .with_custom_attributes(vec!["#[non_exhaustive]"])
                        .with_derive(vec!["std::clone::Clone", "std::cmp::PartialEq", "std::fmt::Debug"])
                        .build()
                        .finish();
                }
                _ => return Result::Err(Error::new(ErrorKind::InvalidInput, format!("Found shape is not a structure '{target}'"))),
            },
            None => return Result::Err(Error::new(ErrorKind::InvalidInput, format!("Unable to find shape '{target}'"))),
        }
        Result::Ok(())
    }

    fn generate_operation(
        writer: &mut CodeWriter, name: &str, operation: &OperationShape, shapes: &HashMap<String, Shape>,
    ) -> Result<(), Error> {
        Result::Ok(())
    }
}
