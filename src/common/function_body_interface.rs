pub mod function_body_interface {
    use swc_ecma_ast::{ArrowExpr, Function, BlockStmtOrExpr};

    pub enum Func<'a> {
        Block(&'a Box<Function>),
        Arrow(&'a ArrowExpr),
    }

    pub trait FunctionBody {
        fn get_body(&self) -> Result<Box<BlockStmtOrExpr>, String>;
    }

    pub trait FunctionAnalysis {
        fn analyze(&self, body: &BlockStmtOrExpr) -> i32;
    }
}
