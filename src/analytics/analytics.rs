pub mod analytics {
    use std::rc::Rc;
    use swc_common::SourceMap;
    use swc_ecma_ast::{ArrowExpr, Function};
    use crate::analytics::report;
    use crate::common::function_like::FunctionLike;

    pub const THRESHOLD: i32 = 4;

    pub fn do_analytics(function_likes: Vec<FunctionLike>, source_map: &Rc<SourceMap>) {
        for function_like in function_likes {
            let score = match function_like {
                FunctionLike::FunctionDecl(fn_decl) => _score_function(&fn_decl.function),
                FunctionLike::FunctionExpr(fn_expr) => _score_function(&fn_expr.function),
                FunctionLike::ArrowFunction(arrow) => _score_function_arrow(arrow),
                FunctionLike::Method(method) => _score_function(&method.function),
                FunctionLike::PropMethod(method) => _score_function(&method.function),
            };

            if score > THRESHOLD {
                report::report_function(function_like, score, source_map);
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
