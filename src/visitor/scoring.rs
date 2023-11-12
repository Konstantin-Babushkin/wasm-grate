pub mod scoring {
    use swc_common::Span;
    use crate::common::create_config_directory;
    use crate::visitor::{FunctionAnalysisVisitor, report};
    use crate::visitor::max_scores::max_scores::*;

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
        // Calculate individual metric scores
        let cyclomatic_score = _calculate_metric_score(visitor.cyclomatic_complexity as f32, visitor.thresholds.cyclomatic_complexity as f32, WEIGHT_CYCLOMATIC);
        let loop_score = _calculate_metric_score(visitor.max_loop_depth as f32, visitor.thresholds.loop_depth as f32, WEIGHT_LOOP);
        let arithmetic_score = _calculate_metric_score(visitor.arithmetic_operations as f32, visitor.thresholds.arithmetic_operations as f32, WEIGHT_ARITHMETIC);

        let string_penalty = if visitor.string_operations > visitor.thresholds.string_operations {
            2.0 * (visitor.string_operations as f32 - visitor.thresholds.string_operations as f32)
        } else {
            0.0
        };

        // Sum the maximum scores
        let max_possible_score = _get_max_possible_score(visitor);

        // Normalize the final score
        let raw_score = cyclomatic_score + loop_score + arithmetic_score - string_penalty;
        let normalized_score = (raw_score as f32 / max_possible_score * 10.0).min(10.0).round() as usize;

        normalized_score
    }

    fn _get_max_possible_score(visitor: &FunctionAnalysisVisitor) -> f32 {
        // Attempt to load existing max scores
        let max_possible_score = match create_config_directory() {
            Ok(config_dir) => {
                if let Some(loaded_scores) = load_max_scores(config_dir.clone()) {
                    // Calculate the total max possible score from loaded scores
                    loaded_scores.max_cyclomatic_score +
                        loaded_scores.max_loop_score +
                        loaded_scores.max_arithmetic_score -
                        loaded_scores.max_string_penalty
                } else {
                    // If loading is unsuccessful, calculate and store new max scores
                    let max_cyclomatic_score = _calculate_max_metric_score(visitor.thresholds.cyclomatic_complexity, WEIGHT_CYCLOMATIC);
                    let max_loop_score = _calculate_max_metric_score(visitor.thresholds.loop_depth, WEIGHT_LOOP);
                    let max_arithmetic_score = _calculate_max_metric_score(visitor.thresholds.arithmetic_operations, WEIGHT_ARITHMETIC);
                    let max_string_penalty = 2.0 * visitor.thresholds.string_operations as f32;

                    let max_scores = MaxScores {
                        max_cyclomatic_score,
                        max_loop_score,
                        max_arithmetic_score,
                        max_string_penalty,
                    };

                    if let Err(e) = store_max_scores(&max_scores, config_dir) {
                        eprintln!("Error writing max scores: {}", e);
                    }

                    // total max possible score
                    max_cyclomatic_score + max_loop_score + max_arithmetic_score - max_string_penalty
                }
            },
            Err(e) => {
                eprintln!("Error creating configuration directory: {}", e);
                0.0 // Default value if unable to create directory or load scores
            },
        };

        return max_possible_score
    }


    fn _calculate_metric_score(actual: f32, threshold: f32, weight: f32) -> f32 {
        (actual - threshold) * weight
    }

    fn _calculate_max_metric_score(threshold: usize, weight: f32) -> f32 {
        threshold as f32 * weight
    }
}