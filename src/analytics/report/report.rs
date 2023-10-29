pub mod report {
    use std::env;
    use std::path::Path;
    use colored::Colorize;
    use swc_common::{FileName, SourceMap};
    use swc_ecma_ast::{FnDecl, Function};

    pub fn report_function(func_decl: &FnDecl, function: &Box<Function>, score: i32, source_map: &SourceMap) {
        let start = function.span.lo();
        let end = function.span.hi();

        let start_location = source_map.lookup_char_pos(start);
        let end_location = source_map.lookup_char_pos(end);

        // Extract the file name
        let file_name = _get_relative_path(&start_location.file.name);

        // Extract the function name
        let function_name = func_decl.ident.sym.as_ref();

        println!(
            "function '{}' has complexity {}/10.\n      \
        Location: '{}' from {}:{} to {}:{}\n",
            function_name.blue().bold(), score.to_string().bold(),
            file_name,
            start_location.line, start_location.col_display,
            end_location.line, end_location.col_display,
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
}
