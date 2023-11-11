extern crate clap;

use clap::{Arg, ArgAction, Command};
use crate::common::Metrics;

mod parsing;
mod common;
mod visitor;

fn main() {
    let cmd = Command::new("wasm-grate")
        .version("0.1.22")
        .author("Konstantin Babushkin: constant.babushkin@gmail.com")
        .about("Analyzes JS projects for potential WebAssembly migration points.")
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .value_name("FILE_OR_DIRECTORY")
                .help("Sets the input file or directory to analyze")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .action(ArgAction::Set)
                .required(true)
        ).get_matches();

    let input_path: String = cmd.get_one::<String>("path").unwrap().to_string();

    let thresholds = Metrics::new();

    parsing::process_input(input_path, &thresholds);
}
