use crate::Rect;
use crate::Scene;
use crate::SpriteInfo;
use crate::constants::{
    MUSIC_ID_PACHEBAL, SFX_ID_BLIP, SFX_ID_MEME, TEXTURE_ID_FONTSHEET, TEXTURE_ID_LEEKSHEET,
    TEXTURE_ID_TITLE_BG, sprite_info_title,
};
use crate::game::GameContext;
use crate::grid_layout::GridLayout;
use crate::renderer::RenderCommand;
use crate::{ReadyState, advance_ready_state};
use crate::scene::button::Button;

pub struct TitleScene {
    bg: SpriteInfo,
    start_game_btn: Button,
    quit_btn: Button,
    played_intro: ReadyState,
    play_music: ReadyState,
}

impl TitleScene {
    fn layout(game_context: &GameContext) -> GridLayout {
        let (screen_width, screen_height) = game_context.screen_size;
        GridLayout {
            area: Rect::new(0, 0, screen_width as isize, screen_height as isize),
            rows: 27,
            columns: 48,
            cell_gap: 0,
        }
    }
}

impl Default for TitleScene {
    fn default() -> TitleScene {
        TitleScene {
            bg: sprite_info_title(),
            //320, 150
            start_game_btn: Button::new(
                "Start".to_string(),
                Rect {
                    x: 32,
                    y: 15,
                    width: 10,
                    height: 2,
                },
            ),
            quit_btn: Button::new(
                "Quit".to_string(),
                Rect {
                    x: 32,
                    y: 18,
                    width: 10,
                    height: 2,
                },
            ),
            played_intro: ReadyState::Ready,
            play_music: ReadyState::Cooldown {
                wait_for: 180,
                ticks_waited: 0,
            },
        }
    }
}

impl Scene for TitleScene {
    fn init(&mut self, game_context: &mut GameContext) {
        let Some(ref mut asset_loader) = game_context.asset_loader else {
            return;
        };

        asset_loader.ensure_texture_spritesheet_loaded(TEXTURE_ID_TITLE_BG);
        asset_loader.ensure_texture_spritesheet_loaded(TEXTURE_ID_LEEKSHEET);
        asset_loader.ensure_texture_spritesheet_loaded(TEXTURE_ID_FONTSHEET);

        let Some(ref mut audio) = game_context.audio else {
            return;
        };

        audio.load_sfx(SFX_ID_BLIP);
        audio.load_sfx(SFX_ID_MEME);
        audio.load_music(MUSIC_ID_PACHEBAL);
    }
    fn update(&mut self, ticks: u32, game_context: &mut GameContext) {
        let layout = TitleScene::layout(&game_context);
        self.quit_btn.update(ticks, game_context, &layout);
        self.start_game_btn.update(ticks, game_context, &layout);

        match self.played_intro {
            ReadyState::Ready => {
                self.played_intro = ReadyState::Cooldown {
                    wait_for: u32::MAX,
                    ticks_waited: 0,
                };
                game_context.audio.as_mut().map(|audio| {
                    audio.play_sfx(SFX_ID_MEME);
                });
            }
            _ => {}
        }

        match self.play_music {
            ReadyState::Ready => {
                game_context.audio.as_mut().map(|audio| {
                    let _ = audio.play_music(MUSIC_ID_PACHEBAL);
                });
                self.play_music = ReadyState::Cooldown {
                    wait_for: 14 * 60,
                    ticks_waited: 0,
                };
            }
            _ => {}
        }

        self.played_intro = advance_ready_state(self.played_intro, ticks);
        self.play_music = advance_ready_state(self.play_music, ticks);

        if self.start_game_btn.clicked && game_context.next_scene.is_none() {
            game_context.audio.as_mut().map(|audio| {
                audio.play_sfx(SFX_ID_BLIP);
            });
            game_context.queue_level();
            self.start_game_btn.clicked = false;
        }

        if self.quit_btn.clicked && game_context.next_scene.is_none() {
            game_context.audio.as_mut().map(|audio| {
                audio.play_sfx(SFX_ID_BLIP);
            });
            game_context.shutdown();
            self.quit_btn.clicked = false;
        }
    }
    fn draw(&mut self, game_context: &mut GameContext) {
        let layout = TitleScene::layout(&game_context);
        let Some(ref mut renderer) = game_context.renderer else {
            return;
        };

        let src = self.bg.get_rect();
        renderer.send_command(RenderCommand::DrawRect {
            texture_id: TEXTURE_ID_TITLE_BG,
            source: src,
            destination: Rect::new(0, 0, layout.area.width, layout.area.height),
        });

        self.quit_btn.draw(game_context, &layout);
        self.start_game_btn.draw(game_context, &layout);
    }
}
