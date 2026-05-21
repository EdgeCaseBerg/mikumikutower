use crate::constants::{MusicId, SfxId};

pub trait Audio {
    fn play_sfx(&mut self, id: SfxId);
    fn load_sfx(&mut self, id: SfxId);
    fn play_music(&mut self, id: MusicId);
    fn load_music(&mut self, id: MusicId);
    fn prepare(&mut self); // <-- dont forget to tlak about adding that
}
