pub mod scoring {
    use swc_common::Span;
    use crate::visitor::{FunctionAnalysisVisitor, report};

    pub fn try_report_function(visitor: &FunctionAnalysisVisitor, span: Span) {
        let metrics_score = _get_metrics_score(visitor);
        if metrics_score > visitor.thresholds.average() {
            report::report_function(span, &visitor.source_map, metrics_score);
        }
    }

    const WEIGHT_CYCLOMATIC: f32 = 0.5;
    const WEIGHT_ARITHMETIC: f32 = 1.0;
    const WEIGHT_LOOP: f32 = 1.5;

    fn _get_metrics_score(visitor: &FunctionAnalysisVisitor) -> usize {
        let FunctionAnalysisVisitor {
            cyclomatic_complexity,
            max_loop_depth,
            arithmetic_operations,
            string_operations,
            thresholds,
            ..
        } = visitor;

        let cyclomatic_score = _calculate_metric_score(*cyclomatic_complexity as f32, thresholds.cyclomatic_complexity as f32, WEIGHT_CYCLOMATIC);
        let loop_score = _calculate_metric_score(*max_loop_depth as f32, thresholds.loop_depth as f32, WEIGHT_LOOP);
        let arithmetic_score = _calculate_metric_score(*arithmetic_operations as f32, thresholds.arithmetic_operations as f32, WEIGHT_ARITHMETIC);

        let string_penalty = if *string_operations > thresholds.string_operations {
            2.0 * (*string_operations as f32 - thresholds.string_operations as f32)
        } else {
            0.0
        };

        // Calculate and normalize the final score
        let raw_score = cyclomatic_score + loop_score + arithmetic_score - string_penalty;
        let range = (10 - visitor.thresholds.cyclomatic_complexity) / 3;
        let max_possible_score = visitor.thresholds.cyclomatic_complexity + 2 * range;

        // Normalize the raw score to be within 0-10
        let normalized_score = if max_possible_score != 0 {
            (raw_score as f32 / max_possible_score as f32 * 10.0).min(10.0).round() as usize
        } else {
            0
        };

        normalized_score
    }

    fn _calculate_metric_score(actual: f32, threshold: f32, weight: f32) -> f32 {
        let score = actual - threshold;
        (score * weight).max(0.0)
    }
}