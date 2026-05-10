use crate::SpriteInfo;

pub const TEXTURE_ID_MIKU: usize = 0;
pub const TEXTURE_ID_PORTRAIT: usize = 1;
pub const TEXTURE_ID_LEEKSHEET: usize = 2;
pub const TEXTURE_ID_FONTSHEET: usize = 3;

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

pub const fn sprite_info_miku() -> SpriteInfo {
    SpriteInfo {
        start_x: 0,
        start_y: 0,
        width: 71,
        height: 54,
        frames: 6,
        current_frame: 0,
        framerate_per_second: 10,
        delta: 0,
    }
}

pub const fn sprite_info_highlight() -> SpriteInfo {
    SpriteInfo {
        start_x: 96,
        start_y: 0,
        width: 32,
        height: 32,
        frames: 4,
        current_frame: 0,
        framerate_per_second: 4,
        delta: 0,
    }
}

pub const fn sprite_info_miku_tower() -> SpriteInfo {
    SpriteInfo {
        start_x: 32 * 7,
        start_y: 0,
        width: 32,
        height: 32,
        frames: 2,
        current_frame: 0,
        framerate_per_second: 8,
        delta: 0,
    }
}

pub const fn sprite_info_rin_tower() -> SpriteInfo {
    SpriteInfo {
        start_x: 32 * 9,
        start_y: 0,
        width: 32,
        height: 32,
        frames: 2,
        current_frame: 0,
        framerate_per_second: 4,
        delta: 0,
    }
}

pub const fn sprite_info_luka_tower() -> SpriteInfo {
    SpriteInfo {
        start_x: 32 * 11,
        start_y: 0,
        width: 32,
        height: 32,
        frames: 2,
        current_frame: 0,
        framerate_per_second: 16,
        delta: 0,
    }
}

pub const fn sprite_info_teto() -> SpriteInfo {
    SpriteInfo {
        start_x: 32 * 13,
        start_y: 0,
        width: 32,
        height: 32,
        frames: 1,
        current_frame: 0,
        framerate_per_second: 16,
        delta: 0,
    }
}

pub const fn sprite_info_teto_walking() -> SpriteInfo {
    SpriteInfo {
        start_x: 32 * 14,
        start_y: 0,
        width: 32,
        height: 32,
        frames: 2,
        current_frame: 0,
        framerate_per_second: 16,
        delta: 0,
    }
}

pub const fn sprite_info_energy() -> SpriteInfo {
    SpriteInfo {
        start_x: 32 * 16,
        start_y: 0,
        width: 32,
        height: 32,
        frames: 1,
        current_frame: 0,
        framerate_per_second: 60,
        delta: 0,
    }
}
