use std::path::PathBuf;

pub struct GameOptions {
    pub name: String,
    pub window_width: u32,
    pub window_height: u32,
    pub assets_path: PathBuf,
    // Eventually we can add log level when we add a real logger in
}

impl Default for GameOptions {
    fn default() -> Self {
        Self {
            name: "Miku Miku Tower".to_owned(),
            window_width: 1280,
            window_height: 720,
            assets_path: PathBuf::new().join("assets"), // relative path so backends can join against it with current_dir implementations
        }
    }
}
