use std::{
    fs::File,
    io::{Error, Read},
};

use clap::{Arg, ArgAction, Command};

use crate::smithy::smithy::Smithy;
use crate::smithy_codegen::SmithyCodegen;

const VERSION: &str = env!("CARGO_PKG_VERSION");

mod codegen;
mod naming;
mod smithy;
mod smithy_codegen;

fn main() -> std::io::Result<()> {
    let matches = Command::new("local_cloud_smithy_gen")
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
        let base_dir = &format!("{base_path}/src/aws");
        let smithy_codegen = SmithyCodegen::new(base_dir, "crate::aws", smithy_shapes);
        smithy_codegen.generate_types()?;
        smithy_codegen.generate_operations()?;
        Result::Ok(())
    }
}
