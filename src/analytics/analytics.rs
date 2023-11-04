pub mod analytics {
    use std::rc::Rc;
    use swc_common::SourceMap;
    use swc_ecma_ast::{ArrowExpr, Function};
    use crate::analytics::report;
    use crate::common::function_like::FunctionLike;

    const THRESHOLD: i32 = 5;

    pub fn do_analytics(function_likes: Vec<FunctionLike>, source_map: &Rc<SourceMap>) {
        for function_like in function_likes {
            let score = match function_like {
                FunctionLike::FunctionDecl(fn_decl) => _score_function(&fn_decl.function),
                FunctionLike::FunctionExpr(fn_expr) => _score_function(&fn_expr.function),
                FunctionLike::ArrowFunction(arrow) => _score_function_arrow(arrow),
                FunctionLike::Method(method) => _score_function(&method.function),
            };

            if score > THRESHOLD {
                match function_like {
                    FunctionLike::FunctionDecl(fn_decl) => {
                        report::report_function(fn_decl, &fn_decl.function, score, source_map)
                    }
                    FunctionLike::FunctionExpr(fn_expr) => {
                        // Handle reporting for function expressions
                    }
                    FunctionLike::ArrowFunction(arrow) => {
                        // Handle reporting for arrow functions
                    }
                    FunctionLike::Method(method) => {
                        // Handle reporting for methods
                    }
                }
            }
        }
    }

    fn _score_function_arrow(_arrow: &ArrowExpr) -> i32 {
        5
    }

    fn _score_function(_function: &Box<Function>) -> i32 {
        10
    }
}
