pub mod loop_analysis {
    use swc_ecma_ast::Stmt;
    use crate::visitor::FunctionAnalysisVisitor;

    pub fn extract_loop_body(stmt: &Stmt) -> Option<&Box<Stmt>> {
        match stmt {
            Stmt::For(for_stmt) => Some(&for_stmt.body),
            Stmt::While(while_stmt) => Some(&while_stmt.body),
            Stmt::DoWhile(do_while_stmt) => Some(&do_while_stmt.body),
            _ => None,
        }
    }

    pub fn handle_loop(visitor: &mut FunctionAnalysisVisitor, body: &Box<Stmt>) {
        visitor.current_loop_depth += 1;
        visitor.max_loop_depth = visitor.max_loop_depth.max(visitor.current_loop_depth);

        _process_loop_body(body, visitor);

        visitor.current_loop_depth -= 1;
    }

    fn _process_loop_body(body: &Box<Stmt>, visitor: &mut FunctionAnalysisVisitor) {
        if let Stmt::Block(block_stmt) = &**body {
            for inner_stmt in &block_stmt.stmts {
                visitor.analyze_statement(inner_stmt);
            }
        } else {
            visitor.analyze_statement(body);
        }
    }

}