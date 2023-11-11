

pub mod function_analysis_visitor {
    use swc_ecma_ast::{ArrowExpr, ClassMethod, BlockStmt, Expr, BlockStmtOrExpr, FnDecl, FnExpr, Stmt};
    use swc_ecma_visit::{Visit, VisitWith};
    use crate::visitor::loop_analysis::loop_analysis;
    use crate::visitor::cyclomatic_complexity::cyclomatic_complexity;


    pub struct FunctionAnalysisVisitor {
        pub cyclomatic_complexity: usize,
        pub current_loop_depth: usize,
        pub max_loop_depth: usize,
        pub arithmetic_operations: usize,
        string_operations: usize,
    }

    impl FunctionAnalysisVisitor {
        pub fn new() -> Self {
            FunctionAnalysisVisitor {
                cyclomatic_complexity: 0,
                current_loop_depth: 0,
                max_loop_depth: 0,
                arithmetic_operations: 0,
                string_operations: 0,
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

            // Handle loops
            if let Some(loop_body) = loop_analysis::extract_loop_body(stmt) {
                loop_analysis::handle_loop(self, loop_body);
            } else {
                stmt.visit_children_with(self);
            }
        }

        pub fn analyze_expression(&mut self, expr: &Expr) {
            cyclomatic_complexity::analyze_expression(self, expr);

            // Skip loop_analysis. No need to check the expression for loops


        }
    }

    impl Visit for FunctionAnalysisVisitor {
        fn visit_fn_decl(&mut self, n: &FnDecl) {
            self.reset_metrics();

            // Analyze the function body
            if let Some(body) = &n.function.body {
                self.analyze_function_body(body);
            }

            // After visiting the function, report the analysis results
            // TODO threshold
            println!("Function {}: Cyclomatic Complexity = {}, Loop Depth = {}, Arithmetic Operations = {}, String Operations = {}",
                     n.ident, self.cyclomatic_complexity, self.max_loop_depth, self.arithmetic_operations, self.string_operations);

            n.visit_children_with(self);
        }

        fn visit_fn_expr(&mut self, n: &FnExpr) {
            self.reset_metrics();

            if let Some(body) = &n.function.body {
                for stmt in &body.stmts {
                    self.analyze_statement(stmt); // Analyze each statement
                }
            }

            let expr_name = n.ident.as_ref().map(|ident| ident.sym.to_string());

            println!("Function Expression {}: Cyclomatic Complexity = {}, Loop Depth = {}, Arithmetic Operations = {}, String Operations = {}",
                     expr_name.unwrap_or_else(|| "Anonymous".to_string()), self.cyclomatic_complexity, self.max_loop_depth, self.arithmetic_operations, self.string_operations);

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

            println!("Arrow Function: Cyclomatic Complexity = {}, Loop Depth = {}, Arithmetic Operations = {}, String Operations = {}",
                     self.cyclomatic_complexity, self.max_loop_depth, self.arithmetic_operations, self.string_operations);

            n.visit_children_with(self);
        }

        fn visit_class_method(&mut self, n: &ClassMethod) {
            self.reset_metrics();
            println!("Visited a class method with name {:?}", n.key);
            // Perform your checks or call functions here...
            n.visit_children_with(self);
        }
    }
}