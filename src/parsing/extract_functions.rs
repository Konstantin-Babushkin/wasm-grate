pub mod extract_functions {
    use swc_ecma_ast::{Decl, Prop, Expr, ModuleDecl, ModuleItem, PropOrSpread, Stmt, VarDecl, ClassDecl, CallExpr, ObjectLit, ClassMember};
    use crate::common::function_like::FunctionLike;

    pub fn extract_functions<'a>(items: &'a [ModuleItem], function_likes: &mut Vec<FunctionLike<'a>>) {
        for item in items {
            match item {
                ModuleItem::Stmt(Stmt::Decl(decl)) => _handle_declaration(decl, function_likes),
                ModuleItem::Stmt(Stmt::Expr(expr_stmt)) => _extract_from_expression(&expr_stmt.expr, function_likes),
                ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(export_decl)) => _handle_declaration(&export_decl.decl, function_likes),
                _ => {}
            }
        }
    }

    // *** HANDLERS ***
    fn _handle_declaration<'a>(decl: &'a Decl, function_likes: &mut Vec<FunctionLike<'a>>) {
        match decl {
            Decl::Fn(fn_decl) => function_likes.push(FunctionLike::FunctionDecl(fn_decl)),
            Decl::Var(var_decl) => _extract_from_variable_declaration(var_decl, function_likes),
            Decl::Class(class_decl) => _extract_from_class_declaration(class_decl, function_likes),
            _ => {}
        }
    }

    fn _extract_from_expression<'a>(expr: &'a Expr, function_likes: &mut Vec<FunctionLike<'a>>) {
        match expr {
            Expr::Fn(fn_expr) => function_likes.push(FunctionLike::FunctionExpr(fn_expr)),
            Expr::Arrow(arrow_expr) => function_likes.push(FunctionLike::ArrowFunction(arrow_expr)),
            Expr::Call(call_expr) => _extract_from_call_expression(call_expr, function_likes),
            Expr::Object(obj_expr) => _extract_from_object_expression(obj_expr, function_likes),
            _ => {}
        }
    }


    // *** HELPERS ***
    fn _extract_from_variable_declaration<'a>(var_decl: &'a VarDecl, function_likes: &mut Vec<FunctionLike<'a>>) {
        for declarator in &var_decl.decls {
            if let Some(init) = &declarator.init {
                _extract_from_expression(init, function_likes);
            }
        }
    }

    fn _extract_from_class_declaration<'a>(class_decl: &'a ClassDecl, function_likes: &mut Vec<FunctionLike<'a>>) {
        for member in &class_decl.class.body {
            if let ClassMember::Method(method_prop) = member {
                function_likes.push(FunctionLike::Method(method_prop));
            }
        }
    }

    fn _extract_from_call_expression<'a>(call_expr: &'a CallExpr, function_likes: &mut Vec<FunctionLike<'a>>) {
        for arg in &call_expr.args {
            _extract_from_expression(&arg.expr, function_likes);
        }
    }

    fn _extract_from_object_expression<'a>(obj_expr: &'a ObjectLit, function_likes: &mut Vec<FunctionLike<'a>>) {
        for prop in &obj_expr.props {
            if let PropOrSpread::Prop(boxed_prop) = prop {
                match &**boxed_prop {
                    Prop::Method(method_prop) => function_likes.push(FunctionLike::PropMethod(method_prop)),
                    Prop::KeyValue(key_value_prop) => _extract_from_expression(&key_value_prop.value, function_likes),
                    _ => {}
                }
            }
        }
    }
}