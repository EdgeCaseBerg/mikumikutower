pub trait AssetLoader {
    fn ensure_texture_spritesheet_loaded(&mut self, sheet_id: usize);
}
