

pub mod function_analysis_visitor {
    use std::rc::Rc;
    use swc_common::{SourceMap, Spanned};
    use swc_ecma_ast::{ArrowExpr, BlockStmt, Expr, BlockStmtOrExpr, FnDecl, FnExpr, Stmt};
    use swc_ecma_visit::{Visit, VisitWith};
    use crate::Metrics;
    use crate::visitor::loop_analysis::loop_analysis;
    use crate::visitor::cyclomatic_complexity::cyclomatic_complexity;
    use crate::visitor::scoring::scoring::try_report_function;
    use crate::visitor::string_counter::string_counter;


    pub struct FunctionAnalysisVisitor<'a> {
        pub cyclomatic_complexity: usize,
        pub current_loop_depth: usize,
        pub max_loop_depth: usize,
        pub arithmetic_operations: usize,
        pub string_operations: usize,
        pub thresholds: &'a Metrics,
        pub source_map: Rc<SourceMap>,
    }

    impl<'a> FunctionAnalysisVisitor<'a> {
        pub fn new(thresholds: &'a Metrics, source_map: Rc<SourceMap>) -> Self {
            FunctionAnalysisVisitor {
                cyclomatic_complexity: 1,
                current_loop_depth: 0,
                max_loop_depth: 0,
                arithmetic_operations: 0,
                string_operations: 0,
                thresholds,
                source_map,
            }
        }

        pub fn reset_metrics(&mut self) {
            self.cyclomatic_complexity = 1;
            self.current_loop_depth = 0;
            self.max_loop_depth = 0;
            self.arithmetic_operations = 0;
            self.string_operations = 0;
        }

        fn analyze_function_body(&mut self, body: &BlockStmt) {
            for stmt in &body.stmts {
                self.analyze_statement(stmt);
            }
        }

        pub fn analyze_statement(&mut self, stmt: &Stmt) {
            cyclomatic_complexity::analyze_statement(stmt, &mut self.cyclomatic_complexity);
            string_counter::analyze_statement(self, stmt);

            // Handle loops
            if let Some(loop_body) = loop_analysis::extract_loop_body(stmt) {
                loop_analysis::handle_loop(self, loop_body);
            } else {
                stmt.visit_children_with(self);
            }
        }

        pub fn analyze_expression(&mut self, expr: &Expr) {
            cyclomatic_complexity::analyze_expression(self, expr);
            string_counter::analyze_expression(self, expr);
            // Skip loop_analysis. No need to check the expression for loops
        }
    }

    impl<'a> Visit for FunctionAnalysisVisitor<'a> {
        fn visit_fn_decl(&mut self, n: &FnDecl) {
            self.reset_metrics();

            // Analyze the function body
            if let Some(body) = &n.function.body {
                self.analyze_function_body(body);
            }

            try_report_function(&self, n.span());
            n.visit_children_with(self);
        }

        fn visit_fn_expr(&mut self, n: &FnExpr) {
            self.reset_metrics();

            if let Some(body) = &n.function.body {
                for stmt in &body.stmts {
                    self.analyze_statement(stmt); // Analyze each statement
                }
            }

            try_report_function(&self, n.span());
            n.visit_children_with(self);
        }

        fn visit_arrow_expr(&mut self, n: &ArrowExpr) {
            self.reset_metrics();

            // Arrow functions can have a body that is either a block statement or a single expression
            match &*n.body {
                BlockStmtOrExpr::BlockStmt(block_stmt) => {
                    for stmt in &block_stmt.stmts {
                        self.analyze_statement(stmt);
                    }
                }
                BlockStmtOrExpr::Expr(expr) => {
                    self.analyze_expression(expr);
                }
            }

            try_report_function(&self, n.span());
            n.visit_children_with(self);
        }
    }
}