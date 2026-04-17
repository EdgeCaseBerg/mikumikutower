use crate::SpriteInfo;

pub const TEXTURE_ID_MIKU: usize = 0;
pub const TEXTURE_ID_PORTRAIT: usize = 1;
pub const TEXTURE_ID_LEEKSHEET: usize = 2;

pub const fn sprite_info_leek() -> SpriteInfo {
    SpriteInfo {
        start_x: 0,
        start_y: 0,
        width: 32,
        height: 32,
        frames: 1,
        current_frame: 0,
        framerate_per_second: 60,
        delta: 0,
    }
}

pub const fn sprite_info_grass() -> SpriteInfo {
    SpriteInfo {
        start_x: 64,
        start_y: 0,
        width: 32,
        height: 32,
        frames: 1,
        current_frame: 0,
        framerate_per_second: 60,
        delta: 0,
    }
}

pub const fn sprite_info_road() -> SpriteInfo {
    SpriteInfo {
        start_x: 32,
        start_y: 0,
        width: 32,
        height: 32,
        frames: 1,
        current_frame: 0,
        framerate_per_second: 60,
        delta: 0,
    }
}
