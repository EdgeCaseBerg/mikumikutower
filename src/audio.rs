use crate::constants::{MusicId, SfxId};
use std::time::Duration;

pub trait Audio {
    fn play_sfx(&mut self, id: SfxId);
    fn load_sfx(&mut self, id: SfxId);
    fn play_music(&mut self, id: MusicId);
    fn load_music(&mut self, id: MusicId);
    fn music_duration_seconds(&self, id: MusicId) -> Duration;
    fn prepare(&mut self);
}
