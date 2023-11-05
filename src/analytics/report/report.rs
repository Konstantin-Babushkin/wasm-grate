pub mod report {
    use std::env;
    use std::path::Path;
    use colored::{Color, Colorize};
    use swc_common::{FileName, SourceMap};
    use crate::analytics::THRESHOLD;
    use crate::common::function_like::FunctionLike;

    // pub fn report_function(function_like: FunctionLike, score: i32, source_map: &SourceMap) {
    //     let (function_name, span) = function_like.name_and_span();
    //
    //     let start_location = source_map.lookup_char_pos(span.lo());
    //     let end_location = source_map.lookup_char_pos(span.hi());
    //
    //     // Extract the file name
    //     let file_name = _get_relative_path(&start_location.file.name);
    //
    //     println!(
    //         "function '{}' has complexity {}/10.\n      \
    //     Location: '{}' from {} to {}\n",
    //         function_name.blue().bold(), score.to_string().bold(),
    //         file_name,
    //         start_location.line,
    //         end_location.line,
    //     );
    // }

    pub fn report_function(
        function_like: FunctionLike,
        score: i32,
        source_map: &SourceMap
    ) {
        let span = function_like.span();

        let start_location = source_map.lookup_char_pos(span.lo());

        let file_name = _get_relative_path(&start_location.file.name);

        let source_code = &start_location.file.src;

        let start_index = source_map.lookup_byte_offset(span.lo()).pos.0 as usize;

        // Extract the function declaration snippet
        let snippet = _extract_function_declaration(&source_code, start_index);

        println!(
            "{}:{}:{}\n{}\n{}: {}\n",
            file_name,
            start_location.line,
            start_location.col_display,
            _get_colorized_score(score, THRESHOLD),
            "Declaration".blue(),
            snippet.trim()
        );
    }

    fn _get_relative_path(file: &FileName) -> String {
        let file_str = file.to_string();
        if file_str.is_empty() {
            return "Invalid or empty file name provided".to_string();
        }

        // Get the current working directory
        let current_dir = match env::current_dir() {
            Ok(dir) => dir,
            Err(_) => return "Failed to get current directory".to_string(),
        };

        // Convert the file_name to a Path and make it relative
        match Path::new(&file_str).strip_prefix(&current_dir) {
            Ok(relative_path) => relative_path.display().to_string(),
            Err(_) => file_str,
        }
    }

    fn _extract_function_declaration(source_code: &str, start_index: usize) -> String {
        // Find the start of the line by looking for the newline character before the function start
        let line_start = source_code[..start_index]
            .rfind('\n')
            .map(|pos| pos + 1) // Start after the newline character
            .unwrap_or(0); // If no newline is found, start from the beginning of the file

        // Tokens that indicate the start of the function body
        let body_start_tokens = ["{", "=>"];

        // Find the index of the first body start token
        let body_start_index = body_start_tokens
            .iter()
            .filter_map(|token| source_code[start_index..].find(token))
            .map(|pos| pos + start_index) // Find the global position of the token
            .min() // Get the earliest occurrence
            .unwrap_or(source_code.len()); // If no token is found, go to the end of the file

        // Extract the function signature without including the body start token
        let signature = &source_code[line_start..body_start_index].trim_end();
        signature.to_string()
    }

    fn _get_colorized_score(score: i32, threshold: i32) -> String {
        // Calculate the benchmarks
        let range = (10 - threshold) / 3;
        let min_benchmark = threshold;
        let mid_benchmark = threshold + range;
        let max_benchmark = mid_benchmark + range;

        let score_label = format!("Complexity: {}/10", score);

        match score {
            _ if score >= max_benchmark => score_label.color(Color::Red).to_string(),
            _ if score >= mid_benchmark => score_label.color(Color::BrightRed).to_string(),
            _ if score >= min_benchmark => score_label.color(Color::Yellow).to_string(),
            _ => score_label,
        }
    }
}
