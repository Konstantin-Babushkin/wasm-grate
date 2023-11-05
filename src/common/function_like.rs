use swc_common::Span;
use swc_ecma_ast::{ArrowExpr, FnDecl, FnExpr, ClassMethod, MethodProp};
pub enum FunctionLike<'a> {
    FunctionDecl(&'a FnDecl),
    FunctionExpr(&'a FnExpr),
    ArrowFunction(&'a ArrowExpr),
    Method(&'a ClassMethod),
    PropMethod(&'a MethodProp),
}

impl<'a> FunctionLike<'a> {
    pub fn span(&self) -> Span {
        match self {
            FunctionLike::FunctionDecl(fn_decl) => fn_decl.function.span,
            FunctionLike::FunctionExpr(fn_expr) => fn_expr.function.span,
            FunctionLike::ArrowFunction(arrow_expr) => arrow_expr.span,
            FunctionLike::Method(class_method) => class_method.function.span,
            FunctionLike::PropMethod(prop_method) => prop_method.function.span,
        }
    }
}

