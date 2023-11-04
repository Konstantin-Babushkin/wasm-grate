use swc_ecma_ast::{ArrowExpr, FnDecl, FnExpr, MethodProp};
pub enum FunctionLike<'a> {
    FunctionDecl(&'a FnDecl),
    FunctionExpr(&'a FnExpr),
    ArrowFunction(&'a ArrowExpr),
    Method(&'a MethodProp),
}
