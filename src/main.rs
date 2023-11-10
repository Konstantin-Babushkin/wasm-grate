extern crate clap;

use clap::{Arg, ArgAction, Command};

mod parsing;
mod analytics;
mod common;

// fn main() {
//     let cmd = Command::new("wasm-grate")
//         .version("0.1.22")
//         .author("Konstantin Babushkin: constant.babushkin@gmail.com")
//         .about("Analyzes JS projects for potential WebAssembly migration points.")
//         .arg(
//             Arg::new("path")
//                 .short('p')
//                 .long("path")
//                 .value_name("FILE_OR_DIRECTORY")
//                 .help("Sets the input file or directory to analyze")
//                 .value_parser(clap::builder::NonEmptyStringValueParser::new())
//                 .action(ArgAction::Set)
//                 .required(true)
//         ).get_matches();
//
//     let input_path: String = cmd.get_one::<String>("path").unwrap().to_string();
//
//     parsing::process_input(input_path);
// }

use swc_common::{sync::Lrc, Mark, SourceMap, DUMMY_SP, FileName};
use swc_ecma_ast::*;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, EsConfig};
use swc_ecma_visit::{Visit, VisitWith};

struct FunctionVisitor;

impl FunctionVisitor {
    fn analyze_function_body(&self, body: &BlockStmt) {
        // Here you could inspect the body for nested loops, string usage, etc.
        for stmt in &body.stmts {
            match stmt {
                Stmt::For(..) | Stmt::While(..) | Stmt::DoWhile(..) => {
                    println!("Found a loop statement");
                },
                // Stmt::Expr(expr) => {
                //     self.analyze_expression(&expr.expr);
                // },
                // Add more patterns as necessary
                _ => {}
            }
        }
    }

    // fn analyze_expression(&self, expr: &Box<Expr>) {
    //     match &**expr {
    //         Expr::Binary(binary_expr) => {
    //             // Check for mathematical operations
    //             if matches!(binary_expr.op, op!("+") | op!("-") | op!("*") | op!("/")) {
    //                 println!("Found a math operation");
    //             }
    //             // Check for string concatenation
    //             if matches!(binary_expr.op, op!("+")) && matches!(&*binary_expr.left, Expr::Lit(Lit::Str(..))) {
    //                 println!("Found a string operation");
    //             }
    //             // Recursively analyze the left and right expressions
    //             self.analyze_expression(&binary_expr.left);
    //             self.analyze_expression(&binary_expr.right);
    //         },
    //         // Add more patterns as necessary
    //         _ => {}
    //     }
    // }
}


impl Visit for FunctionVisitor {
    fn visit_fn_decl(&mut self, n: &FnDecl) {
        println!("Visited a function declaration with name {:?}", n.ident);
        if let Some(body) = n.function.body.as_ref() {
            self.analyze_function_body(body);
        } else {
            panic!("Function body should be present");
        }
        n.visit_children_with(self);
    }
    fn visit_fn_expr(&mut self, n: &FnExpr) {
        if let Some(ident) = &n.ident {
            println!("Visited a named function expression with name {:?}", ident);
        } else {
            println!("Visited an anonymous function expression");
        }
        // Perform your checks or call functions here...
        n.visit_children_with(self);
    }

    fn visit_arrow_expr(&mut self, n: &ArrowExpr) {
        println!("Visited an arrow function expression");
        // Perform your checks or call functions here...
        n.visit_children_with(self);
    }

    fn visit_class_method(&mut self, n: &ClassMethod) {
        println!("Visited a class method with name {:?}", n.key);
        // Perform your checks or call functions here...
        n.visit_children_with(self);
    }

    // Implement other visit methods as needed...
}

fn main() {
    let source_code = r#"
        class MyClass {
            method1() {
                return 5 + 6
            }
            static method2() {
                return 5 + 6
            }
        }

        function myFunction(x, y) {
            const z = x + y;
            for (let i = 0; i < z; i++){
                for (let j = 0; j < z; j++){
                    for (let u = 0; u < z; u++){
                        x = y + z;
                    }
                }
            }
            return x
        }
        const x = 67;

        const myArrowFunction = (x, y) => x + y;
        const myFunctionExpression = function(x, y) {
            return x + y
        };
    "#;

    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(FileName::Custom("input.js".into()), source_code.into());

    let lexer = Lexer::new(
        Syntax::Es(EsConfig {
            // The configuration options here
            jsx: false,
            decorators: false,
            fn_bind: true,
            export_default_from: true,
            import_attributes: false,
            allow_super_outside_method: false,
            allow_return_outside_function: false,
            auto_accessors: false,
            decorators_before_export: false,
            explicit_resource_management: false,
        }),
        Default::default(),
        StringInput::new(&fm.src, fm.start_pos, fm.end_pos),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    let module = parser.parse_module().expect("Failed to parse module");

    let mut visitor = FunctionVisitor;
    module.visit_with(&mut visitor);
}
