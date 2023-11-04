use swc_ecma_ast::{ArrowExpr, FnDecl, FnExpr, ClassMethod};
pub enum FunctionLike<'a> {
    FunctionDecl(&'a FnDecl),
    FunctionExpr(&'a FnExpr),
    ArrowFunction(&'a ArrowExpr),
    Method(&'a ClassMethod),
}
