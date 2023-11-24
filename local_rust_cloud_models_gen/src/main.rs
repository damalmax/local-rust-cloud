use std::{
    fs::{self, File},
    io::{Error, Read, Write},
};

use clap::{Arg, ArgAction, Command};

use crate::{
    codegen::{visibility::Visibility, writer::CodeWriter},
    smithy::{shape::shape::Shape, smithy::Smithy},
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
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("sync", sync_matches)) => {
            let source: &str = sync_matches.get_one::<String>("source").expect("source is provided");
            println!("Source path is '{source}'");
            let target: &str = sync_matches.get_one::<String>("target").expect("target is provided");
            println!("Target path is '{target}'");

            generate_source_code(source, target)?;
            return Result::Ok(());
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }

    fn generate_source_code(source: &str, target: &str) -> Result<(), Error> {
        let mut file = File::open(source)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // read Smithy model
        let smithy: Smithy = serde_json::from_str(&contents)?;

        let base_path = target.trim_end_matches('/').to_string();
        let target_module = &format!("{base_path}/src/aws/operations");
        fs::create_dir_all(target_module)?;

        let mut mod_list = String::new();
        let mut operations_mod_writer = CodeWriter::new(&mut mod_list);
        for (key, shape) in &smithy.shapes {
            match shape {
                Shape::Operation(operation) => {
                    let operation_name = &utils::nice_name(key);
                    operations_mod_writer.new_mod(Visibility::Public, &operation_name).finish();
                    create_operation_mod_file(target_module, operation_name)?;

                    // let mut shapes_list = String::new();
                    // let shapes_writer = CodeWriter::new(&mut shapes_list);
                }
                _ => continue,
            }
        }
        File::create(format!("{target_module}/mods.rs"))?.write_all(&mod_list.as_bytes())?;
        Result::Ok(())
    }

    fn create_operation_mod_file(base_path: &str, operation_name: &str) -> Result<(), Error> {
        // create folder for operation
        fs::create_dir_all(format!("{base_path}/{operation_name}"))?;

        let mut operation_mod_list = String::new();
        let mut operation_mod_writer = CodeWriter::new(&mut operation_mod_list);
        operation_mod_writer.new_mod(Visibility::Public, "shapes").finish();
        operation_mod_writer.new_mod(Visibility::Public, "operation").finish();
        File::create(format!("{base_path}/{operation_name}/mod.rs"))?.write_all(&operation_mod_list.as_bytes())?;
        Result::Ok(())
    }
}
