pub mod function_body_implementation {
    use swc_ecma_ast::{ BlockStmtOrExpr};
    use crate::common::Func;
    use crate::common::FunctionBody;

    impl<'a> FunctionBody for Func<'a> {
        fn get_body(&self) -> Result<Box<BlockStmtOrExpr>, String> {
            match self {
                Func::Block(function) => {
                    function.body.as_ref()
                        .map(|body| Box::new(BlockStmtOrExpr::BlockStmt(body.clone())))
                        .ok_or_else(|| "Function body is missing".to_string())
                },
                Func::Arrow(arrow) => {
                    let boxed_body = match &*arrow.body {
                        BlockStmtOrExpr::BlockStmt(bs) => Box::new(BlockStmtOrExpr::BlockStmt(bs.clone())),
                        BlockStmtOrExpr::Expr(e) => Box::new(BlockStmtOrExpr::Expr(e.clone())),
                    };
                    Ok(boxed_body)
                },
            }
        }
    }
}
