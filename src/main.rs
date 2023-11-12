extern crate clap;

use clap::{Arg, ArgAction, Command};
use crate::common::{create_config_directory, Metrics};

mod parsing;
mod common;
mod visitor;


use dialoguer::Input;
use crate::common::create_config_dir::create_config_dir::delete_max_scores;

fn interactive_configuration() -> Metrics {
    println!("Enter the thresholds for the metrics (press Enter to use default values):");

    let cyclomatic_complexity: usize = Input::new()
        .with_prompt("Max allowed cyclomatic Complexity")
        .default(3)
        .interact()
        .unwrap();

    let loop_depth: usize = Input::new()
        .with_prompt("Max allowed loop depth")
        .default(1)
        .interact()
        .unwrap();

    let arithmetic_operations: usize = Input::new()
        .with_prompt("Max allowed arithmetic operations")
        .default(3)
        .interact()
        .unwrap();

    let string_operations: usize = Input::new()
        .with_prompt("Max allowed string operations")
        .default(1)
        .interact()
        .unwrap();

    return Metrics {
        cyclomatic_complexity,
        loop_depth,
        arithmetic_operations,
        string_operations,
    }
}

fn main() {
    let cmd = Command::new("wasm-grate")
        .version("0.3.2")
        .author("Konstantin Babushkin: constant.babushkin@gmail.com")
        .about("Analyzes JS projects for potential WebAssembly migration points.")
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .value_name("FILE_OR_DIRECTORY")
                .help("Sets the input file or directory to analyze (use '.' for the current directory)")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .action(ArgAction::Set)
                .required(true)
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .action(ArgAction::SetTrue)
                .help("Enter configuration mode to interactively set metrics thresholds")
        ).get_matches();

    let should_configure_interactively = *cmd.get_one::<bool>("config").unwrap_or(&false);

    let thresholds = if should_configure_interactively {
        // Delete existing max_scores.json file before interactive configuration
        if let Ok(config_dir) = create_config_directory() {
            if let Err(e) = delete_max_scores(config_dir) {
                eprintln!("Error deleting max scores: {}", e);
            }
        }
        interactive_configuration()
    } else {
        Metrics::new()
    };

    let input_path: String = cmd.get_one::<String>("path").unwrap().to_string();

    parsing::process_input(input_path, &thresholds);
}
