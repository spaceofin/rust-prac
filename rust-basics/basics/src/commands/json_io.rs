use std::fs;
use std::path::PathBuf;
use dirs_next::data_local_dir;

pub fn save_json(content: String) -> Result<(), String> {
    let dir: PathBuf = data_local_dir()
        .ok_or("Failed to get local data directory")?
        .join("rust-practice");

    if !dir.exists() {
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    }

    let path = dir.join("test.json");
    fs::write(path, content).map_err(|e| e.to_string())
}

pub fn load_json() -> Result<String, String> {
    let dir: PathBuf = data_local_dir()
        .ok_or("Failed to get local data directory")?
        .join("rust-practice");

    let path = dir.join("test.json");
    fs::read_to_string(path).map_err(|e| e.to_string())
}