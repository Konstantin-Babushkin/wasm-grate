use swc_common::Span;
use swc_ecma_ast::{ArrowExpr, FnDecl, FnExpr, ClassMethod, MethodProp, BlockStmtOrExpr};
use swc_ecma_ast::BlockStmtOrExpr::BlockStmt;
use crate::common::FunctionBody;

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

impl<'a> FunctionBody for FunctionLike<'a> {
    fn get_body(&self) -> Result<Box<BlockStmtOrExpr>, String> {
        match self {
            FunctionLike::FunctionDecl(fn_decl) => {
                fn_decl.function.body.as_ref()
                    .map(|b| Box::new(BlockStmt(b.clone())))
                    .ok_or_else(|| "FunctionDecl body is missing".to_string())
            },
            FunctionLike::FunctionExpr(fn_expr) => {
                fn_expr.function.body.as_ref()
                    .map(|b| Box::new(BlockStmt(b.clone())))
                    .ok_or_else(|| "FunctionExpr body is missing".to_string())
            },
            FunctionLike::ArrowFunction(arrow_expr) => {
                match &*arrow_expr.body {
                    BlockStmt(bs) => Ok(Box::new(BlockStmt(bs.clone()))),
                    BlockStmtOrExpr::Expr(e) => Ok(Box::new(BlockStmtOrExpr::Expr(e.clone()))),
                }
            },
            FunctionLike::Method(method) => {
                method.function.body.as_ref()
                    .map(|b| Box::new(BlockStmt(b.clone())))
                    .ok_or_else(|| "Method body is missing".to_string())
            },
            FunctionLike::PropMethod(prop_method) => {
                prop_method.function.body.as_ref()
                    .map(|b| Box::new(BlockStmt(b.clone())))
                    .ok_or_else(|| "PropMethod body is missing".to_string())
            },
        }
    }
}
