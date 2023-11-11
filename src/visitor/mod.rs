pub mod visitor;
pub use visitor::function_analysis_visitor::FunctionAnalysisVisitor;

mod loop_analysis;
mod cyclomatic_complexity;
mod string_counter;
mod report;
mod scoring;



