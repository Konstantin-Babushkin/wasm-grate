pub mod max_scores {
    use serde::{Serialize, Deserialize};
    use std::fs;
    use std::path::PathBuf;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct MaxScores {
        pub max_cyclomatic_score: f32,
        pub max_loop_score: f32,
        pub max_arithmetic_score: f32,
        pub max_string_penalty: f32,
    }

    pub fn store_max_scores(scores: &MaxScores, config_dir: PathBuf) -> std::io::Result<()> {
        let file_path = config_dir.join("max_scores.json");
        let serialized = serde_json::to_string(scores)?;
        fs::write(file_path, serialized)
    }

    pub fn load_max_scores(config_dir: PathBuf) -> Option<MaxScores> {
        let file_path = config_dir.join("max_scores.json");
        if file_path.exists() {
            let data = fs::read_to_string(file_path).ok()?;
            serde_json::from_str(&data).ok()
        } else {
            None
        }
    }
}