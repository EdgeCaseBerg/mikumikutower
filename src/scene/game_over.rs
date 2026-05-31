use crate::Rect;
use crate::Scene;
use crate::SpriteInfo;
use crate::constants::{
    MUSIC_ID_MOON, MUSIC_ID_TETO, MusicId, SFX_ID_BLIP, TEXTURE_ID_FONTSHEET, TEXTURE_ID_GAMEOVER,
    TEXTURE_ID_LEEKSHEET, sprite_info_gameover_miku,
};
use crate::game::GameContext;
use crate::grid_layout::GridLayout;
use crate::renderer::RenderCommand;
use crate::scene::button::Button;

pub struct GameOverScene {
    miku: SpriteInfo,
    try_again_btn: Button,
    give_up_btn: Button,
    desired_music: MusicId,
    current_music: MusicId,
}

impl GameOverScene {
    fn layout(game_context: &GameContext) -> GridLayout {
        let (screen_width, screen_height) = game_context.screen_size;
        GridLayout {
            area: Rect::new(0, 0, screen_width as isize, screen_height as isize),
            rows: 10,
            columns: 15,
            cell_gap: 0,
        }
    }
}

impl Default for GameOverScene {
    fn default() -> GameOverScene {
        GameOverScene {
            miku: sprite_info_gameover_miku(),
            try_again_btn: Button::new(
                "Continue?".to_string(),
                Rect {
                    x: 2,
                    y: 3,
                    width: 5,
                    height: 1,
                },
            ),
            give_up_btn: Button::new(
                "Give up?".to_string(),
                Rect {
                    x: 8,
                    y: 3,
                    width: 5,
                    height: 1,
                },
            ),
            desired_music: MUSIC_ID_TETO,
            current_music: MUSIC_ID_MOON, // start off wrong to trigger it
        }
    }
}

impl Scene for GameOverScene {
    fn init(&mut self, game_context: &mut GameContext) {
        let Some(ref mut asset_loader) = game_context.asset_loader else {
            return;
        };

        asset_loader.ensure_texture_spritesheet_loaded(TEXTURE_ID_GAMEOVER);
        asset_loader.ensure_texture_spritesheet_loaded(TEXTURE_ID_LEEKSHEET);
        asset_loader.ensure_texture_spritesheet_loaded(TEXTURE_ID_FONTSHEET);

        let Some(ref mut audio) = game_context.audio else {
            return;
        };

        let _ = audio.load_sfx(SFX_ID_BLIP);
        let _ = audio.load_music(MUSIC_ID_MOON);
        let _ = audio.load_music(MUSIC_ID_TETO);
    }

    fn update(&mut self, ticks: u32, game_context: &mut GameContext) {
        let layout = GameOverScene::layout(&game_context);
        self.give_up_btn.update(ticks, game_context, &layout);
        self.try_again_btn.update(ticks, game_context, &layout);

        // Check where mouse is, hover over quit -> miku sobbing (frame 1)
        if self.give_up_btn.hovered {
            self.miku.current_frame = 1;
            self.desired_music = MUSIC_ID_TETO;
        } else if self.try_again_btn.hovered {
            self.miku.current_frame = 2;
            self.desired_music = MUSIC_ID_MOON;
        } else {
            self.miku.current_frame = 0;
        }

        if self.try_again_btn.clicked && game_context.next_scene.is_none() {
            game_context.queue_level();
            game_context.audio.as_mut().map(|audio| {
                let _ =audio.play_sfx(SFX_ID_BLIP);
            });
            self.try_again_btn.clicked = false;
        }

        if self.give_up_btn.clicked && game_context.next_scene.is_none() {
            game_context.shutdown();
            game_context.audio.as_mut().map(|audio| {
                let _ =audio.play_sfx(SFX_ID_BLIP);
            });
            self.give_up_btn.clicked = false;
        }

        if self.desired_music != self.current_music {
            game_context.audio.as_mut().map(|audio| {
                // if we wanted to show an error then we could handle this
                // but audio not playing is fine for now
                let _ = audio.play_music(self.desired_music);
                self.current_music = self.desired_music;
            });
        }
    }
    fn draw(&mut self, game_context: &mut GameContext) {
        let layout = GameOverScene::layout(&game_context);
        self.give_up_btn.draw(game_context, &layout);
        self.try_again_btn.draw(game_context, &layout);
        let Some(ref mut renderer) = game_context.renderer else {
            return;
        };

        let cell = layout.cell_rect(0, 0);

        let src = self.miku.get_rect();
        renderer.send_command(RenderCommand::DrawRect {
            texture_id: TEXTURE_ID_GAMEOVER,
            source: src,
            destination: Rect::new(cell.x, cell.y, layout.area.width, layout.area.height),
        });
    }
}
