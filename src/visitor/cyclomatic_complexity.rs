pub mod cyclomatic_complexity {
    use swc_ecma_ast::{BinaryOp, Expr, Stmt};
    use swc_ecma_visit::VisitWith;
    use crate::visitor::FunctionAnalysisVisitor;

    pub fn analyze_statement(stmt: &Stmt, complexity: &mut usize) {
        match stmt {
            Stmt::If(_) => {
                *complexity += 1;
            },
            Stmt::Switch(switch_stmt) => {
                *complexity += switch_stmt.cases.len();
            },
            Stmt::For(_) | Stmt::While(_) | Stmt::DoWhile(_) => {
                *complexity += 1;
            },
            _ => {}
        }
    }

    pub fn analyze_expression(visitor: &mut FunctionAnalysisVisitor, expr: &Expr) {
        match expr {
            Expr::Bin(bin_expr) => {
                // Check for logical binary operators
                match bin_expr.op {
                    BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div => {
                        visitor.arithmetic_operations += 1;
                    }
                    BinaryOp::LogicalAnd | BinaryOp::LogicalOr => {
                        visitor.cyclomatic_complexity += 1;
                    }
                    _ => {}
                }

                // Recursively analyze the left and right operands
                visitor.analyze_expression(&bin_expr.left);
                visitor.analyze_expression(&bin_expr.right);
            },
            Expr::Cond(cond_expr) => {
                // Ternary operator represents a decision point
                visitor.cyclomatic_complexity += 1;

                // Recursively analyze the condition, consequent, and alternative of ternary operator
                visitor.analyze_expression(&cond_expr.test);
                visitor.analyze_expression(&cond_expr.cons);
                visitor.analyze_expression(&cond_expr.alt);
            },
            Expr::Paren(paren_expr) => {
                visitor.analyze_expression(paren_expr.expr.unwrap_parens());
            }
            _ => {}
        }
        expr.visit_children_with(visitor);
    }
}