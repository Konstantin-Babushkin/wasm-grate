
pub mod parsing {
    use std::path::Path;
    use std::fs;
    use std::rc::Rc;
    use std::ffi::OsStr;
    use swc_common::FileName;
    use swc_common::input::StringInput;
    use swc_common::source_map::SourceMap;
    use swc_ecma_parser::{Parser, Syntax};
    use swc_ecma_ast::{Decl, Prop, Expr, ModuleDecl, ModuleItem, PropOrSpread, Stmt, VarDecl};
    use swc_ecma_ast::ClassMember::Method;
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

        // Extract functions from the module
        let mut function_likes = Vec::new();
        _extract_functions(&module.body, &mut function_likes);
        if function_likes.is_empty() {
            return;
        }
        analytics::do_analytics(function_likes, &source_map);
    }

    fn _extract_functions<'a>(items: &'a [ModuleItem], function_likes: &mut Vec<FunctionLike<'a>>) {
        for item in items {
            match item {
                ModuleItem::Stmt(Stmt::Decl(Decl::Fn(fn_decl))) => {
                    function_likes.push(FunctionLike::FunctionDecl(fn_decl));
                },
                ModuleItem::Stmt(Stmt::Decl(Decl::Var(var_decl))) => {
                    _extract_functions_from_var(var_decl, function_likes);
                },
                ModuleItem::Stmt(Stmt::Expr(expr_stmt)) => {
                    _extract_function_likes_from_expr(&expr_stmt.expr, function_likes);
                },
                ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(export_decl)) => {
                    match &export_decl.decl {
                        Decl::Fn(fn_decl) => {
                            function_likes.push(FunctionLike::FunctionDecl(fn_decl));
                        },
                        Decl::Var(var_decl) => {
                            _extract_functions_from_var(var_decl, function_likes);
                        },
                        _ => {}
                    }
                },
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
    }

    fn _extract_functions_from_var<'a>(var_decl: &'a VarDecl, function_likes: &mut Vec<FunctionLike<'a>>) {
        for declarator in &var_decl.decls {
            if let Some(init) = &declarator.init {
                match &**init {
                    Expr::Fn(fn_expr) => {
                        function_likes.push(FunctionLike::FunctionExpr(fn_expr));
                    },
                    Expr::Arrow(arrow_expr) => {
                        function_likes.push(FunctionLike::ArrowFunction(arrow_expr));
                    },
                    _ => {}
                }
            }
        }
    }

    fn _extract_function_likes_from_expr<'a>(expr: &'a Expr, function_likes: &mut Vec<FunctionLike<'a>>) {
        match expr {
            Expr::Fn(fn_expr) => {
                function_likes.push(FunctionLike::FunctionExpr(fn_expr));
            },
            Expr::Arrow(arrow_expr) => {
                function_likes.push(FunctionLike::ArrowFunction(arrow_expr));
            },
            Expr::Call(call_expr) => {
                // Recursively search within call expressions, as they may contain IIFEs or chained calls
                for arg in &call_expr.args {
                    _extract_function_likes_from_expr(&arg.expr, function_likes);
                }
            },
            Expr::Object(obj_expr) => {
                for prop in &obj_expr.props {
                    match prop {
                        PropOrSpread::Prop(boxed_prop) => {
                            match &**boxed_prop {
                                Prop::Method(method_prop) => {
                                    function_likes.push(FunctionLike::PropMethod(method_prop));
                                },
                                Prop::KeyValue(key_value_prop) => {
                                    _extract_function_likes_from_expr(&key_value_prop.value, function_likes);
                                },
                                _ => {}
                            }
                        },
                        _ => {}
                    }
                }
            },
            _ => {}
        }
    }
}
