pub mod create_config_dir {
    use std::env;
    use std::fs;
    use std::path::PathBuf;

    pub fn create_config_directory() -> std::io::Result<PathBuf> {
        let cwd = env::current_dir()?;
        let config_dir = cwd.join(".wasm-grate-config");

        if !config_dir.exists() {
            fs::create_dir(&config_dir)?;
        }

        Ok(config_dir)
    }
    
    pub fn delete_max_scores(config_dir: PathBuf) -> std::io::Result<()> {
        let file_path = config_dir.join("max_scores.json");
        if file_path.exists() {
            fs::remove_file(file_path)?;
        }
        Ok(())
    }
}
