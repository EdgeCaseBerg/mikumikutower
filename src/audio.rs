use crate::constants::{MusicId, SfxId};
use std::time::Duration;
use std::error::Error;

pub trait Audio {
    fn play_sfx(&mut self, id: SfxId);
    fn load_sfx(&mut self, id: SfxId);
    fn play_music(&mut self, id: MusicId) -> Result<(), Box<dyn Error>>;
    fn load_music(&mut self, id: MusicId);
    fn load_bg_music(&mut self) -> Vec<MusicId>;
    fn music_duration_seconds(&self, id: MusicId) -> Duration;
    fn prepare(&mut self);
}
