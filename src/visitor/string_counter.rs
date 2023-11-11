pub mod string_counter {
    use swc_ecma_ast::{Expr, Stmt, Lit, Decl, CallExpr, MemberExpr, Ident, MemberProp, Callee};
    use swc_ecma_visit::VisitWith;
    use crate::visitor::FunctionAnalysisVisitor;

    pub fn analyze_statement(visitor: &mut FunctionAnalysisVisitor, stmt: &Stmt) {
        match stmt {
            Stmt::Decl(Decl::Var(var_decl)) => {
                for decl in &var_decl.decls {
                    if let Some(init) = &decl.init {
                        // Check if the variable is initialized with a string
                        if matches!(&**init, Expr::Lit(Lit::Str(_))) {
                            visitor.string_operations += 1;
                        }
                        visitor.analyze_expression(init);
                    }
                }
            },
            Stmt::Return(return_stmt) => {
                // Check return statements for strings
                if let Some(return_expr) = &return_stmt.arg {
                    visitor.analyze_expression(return_expr);
                }
            },
            _ => {}
        }
        stmt.visit_children_with(visitor);
    }

    pub fn analyze_expression(visitor: &mut FunctionAnalysisVisitor, expr: &Expr) {
        match expr {
            Expr::Lit(Lit::Str(_)) | Expr::Tpl(_) => {
                visitor.string_operations += 1; // Count string literals and template literals
            },
            Expr::Call(call_expr) => {
                if is_string_method_call(call_expr) {
                    visitor.string_operations += 1;
                }
                // Recursively analyze arguments of the call
                for arg in &call_expr.args {
                    visitor.analyze_expression(&arg.expr);
                }
            },
            _ => {}
        }
        expr.visit_children_with(visitor);
    }


    const STRING_METHODS: [&str; 17] = [
"length", "slice", "substring", "substr", "replace", "replaceAll",
"toUpperCase", "toLowerCase", "concat", "trim", "trimStart", "trimEnd",
"padStart", "padEnd", "charAt", "charCodeAt", "split",
    ];
    fn is_string_method_call(call_expr: &CallExpr) -> bool {
        // First match against Callee
        if let Callee::Expr(expr) = &call_expr.callee {
            // Then check if the Expr is a MemberExpr
            if let Expr::Member(MemberExpr { obj: _, prop, .. }) = &**expr {
                // Check if the property is an Ident (an identifier)
                if let MemberProp::Ident(Ident { sym, .. }) = prop {
                    return STRING_METHODS.contains(&sym.as_ref());
                }
            }
        }
        false
    }
}
