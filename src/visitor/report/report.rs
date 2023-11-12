pub mod report {
    use std::rc::Rc;
    use colored::{Color, Colorize};
    use swc_common::{SourceMap, Span};

    const CONCERN: usize = 3;
    const WARNING: usize = 5;
    const DANGER: usize = 7;

    pub fn report_function(
        span: Span,
        source_map: &Rc<SourceMap>,
        score: usize,
    ) {
        let start_location = source_map.lookup_char_pos(span.lo());

        let file_name = &start_location.file.name.to_string();

        let source_code = &start_location.file.src;

        let start_index = source_map.lookup_byte_offset(span.lo()).pos.0 as usize;

        // Extract the function declaration snippet
        let snippet = _extract_function_declaration(&source_code, start_index);

        println!(
            "{}:{}:{}\n{}\n{}: {}\n",
            file_name,
            start_location.line,
            start_location.col_display,
            _get_colorized_score(score),
            "Declaration".blue(),
            snippet.trim()
        );
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

    fn _get_colorized_score(score: usize) -> String {
        let score_label = format!("Complexity: {}/10", score);

        match score {
            _ if score >= DANGER => score_label.color(Color::Red).to_string(),
            _ if score >= WARNING => score_label.color(Color::BrightRed).to_string(),
            _ if score >= CONCERN => score_label.color(Color::Yellow).to_string(),
            _ => score_label,
        }
    }
}
