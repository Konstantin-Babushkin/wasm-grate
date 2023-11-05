pub mod analytics {
    use std::rc::Rc;
    use swc_common::SourceMap;
    use swc_ecma_ast::{BlockStmtOrExpr};
    use crate::analytics::report;
    use crate::common::function_like_enum::FunctionLike;
    use crate::common::FunctionBody;

    pub const THRESHOLD: i32 = 4;

    pub fn do_analytics<'a>(function_likes: Vec<FunctionLike<'a>>, source_map: &Rc<SourceMap>) {
        for function_like in function_likes {
            let score = match function_like.get_body() {
                Ok(container) => _score_function(container),
                Err(e) => {
                    eprintln!("Error extracting function body: {}", e);
                    continue; // Skip this function_like because the body could not be extracted
                },
            };

            if score > THRESHOLD {
                report::report_function(function_like, score, source_map);
            }
        }
    }

    fn _score_function(container: Box<BlockStmtOrExpr>) -> i32 {
        // do scoring for BlockStmt
        // or do scoring for Expr
        // This is a placeholder, the actual implementation will depend on how you want to score the function
        10
    }




}
