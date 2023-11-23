use std::{fs::File, io::Read};

use clap::{Arg, ArgAction, Command};

use crate::{input::InputWriter, mod_writer::ModWriter, smithy::{shape::shape::Shape, smithy::Smithy}};

const VERSION: &str = env!("CARGO_PKG_VERSION");

mod codegen;
mod input;
mod mod_writer;
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

            let mut file = File::open(source)?;

            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            let smithy: Smithy = serde_json::from_str(&contents)?;
            println!("{:?}", smithy);

            let input_writer = InputWriter::new();
            let mod_writer = ModWriter::new();

            let target_module = target.trim_end_matches('/').to_string() + "/src/aws/operations";

            for (key, shape) in &smithy.shapes {
                match shape {
                    Shape::Operation(_) => {
                        input_writer.write_input_for_operation(&target_module, &key, &smithy.shapes)?;
                        break;
                    }
                    _ => continue,
                }
            }
            return Result::Ok(());
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}
