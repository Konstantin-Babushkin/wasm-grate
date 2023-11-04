
pub mod parsing {
    use std::path::Path;
    use std::fs;
    use std::rc::Rc;
    use std::ffi::OsStr;
    use swc_common::FileName;
    use swc_common::input::StringInput;
    use swc_common::source_map::SourceMap;
    use swc_ecma_parser::{Parser, Syntax};
    use swc_ecma_ast::{Decl, Expr, Module, ModuleDecl, ModuleItem, Stmt};
    use swc_ecma_ast::Prop::Method;
    use crate::analytics;
    use crate::common::function_like::FunctionLike;


    pub fn process_input<P: AsRef<Path>>(path: P) {
        let path = path.as_ref();

        if path == Path::new(".") {
            let current_dir = std::env::current_dir().expect("Failed to get current directory");
            process_input(&current_dir);
        } else if path.is_file() {
            if path.extension().map_or(false, |ext| ext == "js" || ext == "ts") {
                _process_file(path);
            }
        } else if path.is_dir() {
            for entry in fs::read_dir(path).expect("Failed to read directory") {
                let entry = entry.expect("Failed to read directory entry");
                process_input(entry.path());
            }
        }
    }

    fn _process_file<P: AsRef<Path>>(file_path: P) {
        let source_code = fs::read_to_string(&file_path).expect("Failed to read file");

        let source_map: Rc<SourceMap> = Rc::new(SourceMap::default());

        // Register the file with the SourceMap
        let file = source_map.new_source_file(
            FileName::Real(file_path.as_ref().to_path_buf()),
            source_code.clone()
        );

        // Create an input from the source code
        let input = StringInput::new(&source_code, file.start_pos, file.end_pos);

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

        // Extract and analyze function-like constructs
        let function_likes = _extract_function_likes(&module);
        if function_likes.is_empty() {
            return;
        }
        analytics::do_analytics(function_likes, &source_map);
    }

    fn _extract_function_likes(module: &Module) -> Vec<FunctionLike> {
        let mut function_likes = Vec::new();

        for item in &module.body {
            match item {
                // handle functions
                ModuleItem::Stmt(Stmt::Decl(Decl::Fn(fn_decl))) => {
                    function_likes.push(FunctionLike::FunctionDecl(fn_decl));
                },
                // handle function expressions and arrow functions
                ModuleItem::Stmt(Stmt::Expr(expr_stmt)) => {
                    match &*expr_stmt.expr {
                        Expr::Fn(fn_expr) => function_likes.push(FunctionLike::FunctionExpr(fn_expr)),
                        Expr::Arrow(arrow_expr) => function_likes.push(FunctionLike::ArrowFunction(arrow_expr)),
                        _ => {}
                    }
                },
                // handle function declarations
                ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(export_decl)) => {
                    if let Decl::Fn(fn_decl) = &export_decl.decl {
                        function_likes.push(FunctionLike::FunctionDecl(fn_decl));
                    }
                },
                // handle Class methods
                ModuleItem::Stmt(Stmt::Decl(Decl::Class(class_decl))) => {
                    for member in &class_decl.class.body {
                        if let Method(method_prop) = member {
                            function_likes.push(FunctionLike::Method(method_prop));
                        }
                    }
                },
                _ => {}
            }
        }

        function_likes
    }
}
