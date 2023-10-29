extern crate clap;
extern crate swc_ecma_parser;
extern crate swc_common;
extern crate swc_ecma_ast;

use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use clap::{Command, Arg, ArgAction};
use swc_ecma_parser::{Parser, StringInput, Syntax};
use swc_common::{SourceMap, BytePos};
use std::rc::Rc;
use swc_ecma_ast::{Module, ModuleItem, Stmt, Decl, Function};

fn main() {
    let cmd = Command::new("wasm-grate")
        .version("0.1")
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

    process_input(input_path);
}

fn process_input<P: AsRef<Path>>(path: P) {
    let path = path.as_ref();

    if path == Path::new(".") {
        let current_dir = std::env::current_dir().expect("Failed to get current directory");
        process_input(&current_dir);
    } else if path.is_file() {
        if path.extension().map_or(false, |ext| ext == "js" || ext == "ts") {
            process_file(path);
        }
    } else if path.is_dir() {
        for entry in fs::read_dir(path).expect("Failed to read directory") {
            let entry = entry.expect("Failed to read directory entry");
            process_input(entry.path());
        }
    }
}

fn process_file<P: AsRef<Path>>(file_path: P) {
    let source_code = fs::read_to_string(&file_path).expect("Failed to read file");

    let cm: Rc<SourceMap> = Rc::new(SourceMap::default());

    // Create an input from the source code
    let input = StringInput::new(&source_code, BytePos(0), BytePos(source_code.len() as u32));

    let file_extension = file_path.as_ref().extension().and_then(OsStr::to_str);
    let lang_syntax = if file_extension == Some("ts") {
        Syntax::Typescript(Default::default())
    } else {
        Syntax::default()
    };

    // Create a parser
    let mut parser = Parser::new(lang_syntax, input, None);

    // Parse the source code into an AST
    let module = parser.parse_module().expect("Failed to parse module");

    // Extract and analyze functions
    let functions = extract_functions(&module);
    for function in functions {
        let score = score_function(&function);
        if score > THRESHOLD {
            report_function(&function, score, &cm);
        }
    }
}

fn extract_functions(module: &Module) -> Vec<&Box<Function>> {
    let mut functions = Vec::new();
    for item in &module.body {
        if let ModuleItem::Stmt(Stmt::Decl(Decl::Fn(func_decl))) = item {
            functions.push(&func_decl.function);
        }
        // You can also handle FunctionExpr and ArrowFunc here if needed
    }
    functions
}

fn score_function(function: &Box<Function>) -> i32 {
    // Apply your scoring logic based on your criteria
    // For simplicity, this example returns a dummy score
    10
}

fn report_function(function: &Box<Function>, score: i32, cm: &SourceMap) {
    let start = function.span.lo();
    let end = function.span.hi();

    let start_location = cm.lookup_char_pos(start);
    let end_location = cm.lookup_char_pos(end);

    println!(
        "Function from {}:{} to {}:{} has a Score of {}",
        start_location.line, start_location.col_display,
        end_location.line, end_location.col_display,
        score
    );
}

const THRESHOLD: i32 = 5;

