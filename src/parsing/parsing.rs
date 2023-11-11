
pub mod parsing {
    use std::path::Path;
    use std::fs;
    use std::rc::Rc;
    use std::ffi::OsStr;
    use swc_common::FileName;
    use swc_common::input::StringInput;
    use swc_common::source_map::SourceMap;
    use swc_ecma_parser::{Parser, Syntax};
    use swc_ecma_visit::VisitWith;
    use crate::visitor::FunctionAnalysisVisitor;


    pub fn process_input<P: AsRef<Path>>(path: P) {
        let path = path.as_ref();

        if path == Path::new(".") {
            let current_dir = std::env::current_dir().expect("Failed to get current directory");
            process_input(&current_dir);
        } else if path.is_file() {
            if path.extension().map_or(false, |ext| ext == "js" || ext == "ts") {
                _process_file(path);
            }
        } else if path.is_dir() {
            for entry in fs::read_dir(path).expect("Failed to read directory") {
                let entry = entry.expect("Failed to read directory entry");
                process_input(entry.path());
            }
        }
    }

    fn _process_file<P: AsRef<Path>>(file_path: P) {
        let source_code = fs::read_to_string(&file_path).expect("Failed to read file");

        let source_map: Rc<SourceMap> = Rc::new(SourceMap::default());

        // Register the file with the SourceMap
        let file = source_map.new_source_file(
            FileName::Real(file_path.as_ref().to_path_buf()),
            source_code.clone()
        );

        // Create an input from the source code
        let input = StringInput::new(&source_code, file.start_pos, file.end_pos);

        let file_extension = file_path.as_ref().extension().and_then(OsStr::to_str);
        let lang_syntax = if file_extension == Some("ts") {
            Syntax::Typescript(Default::default())
        } else {
            Syntax::default()
        };

        // Create a parser
        let mut parser = Parser::new(lang_syntax, input, None);

        // Parse the source code into an AST
        let module = parser.parse_module().expect("Failed to parse module");

        let mut visitor = FunctionAnalysisVisitor::new();
        module.visit_with(&mut visitor);
    }
}
