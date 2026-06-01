use crate::constants::{MusicId, SfxId};
use std::error::Error;
use std::time::Duration;

pub type AudioResult<T> = Result<T, Box<dyn Error>>;

pub trait Audio {
    fn play_sfx(&mut self, id: SfxId) -> AudioResult<()>;
    fn load_sfx(&mut self, id: SfxId) -> AudioResult<()>;
    fn play_music(&mut self, id: MusicId) -> AudioResult<()>;
    fn load_music(&mut self, id: MusicId) -> AudioResult<()>;
    fn load_bg_music(&mut self) -> Vec<AudioResult<MusicId>>;
    fn music_duration_seconds(&self, id: MusicId) -> AudioResult<Duration>;
    fn prepare(&mut self) -> Vec<AudioResult<()>>;
}
