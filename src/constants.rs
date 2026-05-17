use crate::SpriteInfo;
use std::path::PathBuf;

pub const TEXTURE_ID_MIKU: usize = 0;
pub const TEXTURE_ID_PORTRAIT: usize = 1;
pub const TEXTURE_ID_LEEKSHEET: usize = 2;
pub const TEXTURE_ID_FONTSHEET: usize = 3;
pub const TEXTURE_ID_GAMEOVER: usize = 4;
pub const TEXTURE_ID_MIKU_WAVE: usize = 5;

pub fn id_to_relative_path(id: usize) -> PathBuf {
    match id {
        TEXTURE_ID_LEEKSHEET => PathBuf::new().join("made-by-me").join("leek-bg1-bg2.png"),
        TEXTURE_ID_GAMEOVER => PathBuf::new().join("made-by-me").join("GameOver.png"),
        TEXTURE_ID_PORTRAIT => PathBuf::new()
            .join("chaim-vester")
            .join("portraits-spritesheet.png"),
        TEXTURE_ID_MIKU => PathBuf::new().join("dance.png"),
        TEXTURE_ID_FONTSHEET => PathBuf::new()
            .join("webfontkit-BoldPixels")
            .join("BoldPixels-edit.png"),
        TEXTURE_ID_MIKU_WAVE => PathBuf::new().join("made-by-me").join("miku-wave.png"),
        _ => PathBuf::new(), // could maybe panic here, though stuff like unreachable! isnt a const function
    }
}

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

pub const fn sprite_info_topbar_bg() -> SpriteInfo {
    SpriteInfo {
        start_x: 32 * 17,
        start_y: 0,
        width: 32,
        height: 32,
        frames: 1,
        current_frame: 0,
        framerate_per_second: 60,
        delta: 0,
    }
}

pub const fn sprite_info_gameover_miku() -> SpriteInfo {
    SpriteInfo {
        start_x: 0,
        start_y: 0,
        width: 480,
        height: 320,
        frames: 3,
        current_frame: 0,
        framerate_per_second: 60,
        delta: 0,
    }
}

pub const fn sprite_info_portrait() -> SpriteInfo {
    SpriteInfo {
        start_x: 0,
        start_y: 0,
        width: 2478,
        height: 402,
        frames: 1,
        current_frame: 0,
        framerate_per_second: 60,
        delta: 0,
    }
}

pub const fn sprite_info_miku_wave() -> SpriteInfo {
    SpriteInfo {
        start_x: 0,
        start_y: 0,
        width: 320,
        height: 320,
        frames: 2,
        current_frame: 0,
        framerate_per_second: 30,
        delta: 0,
    }
}
