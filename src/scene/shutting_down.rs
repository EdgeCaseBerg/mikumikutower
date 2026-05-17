use crate::Rect;
use crate::Scene;
use crate::SpriteInfo;
use crate::constants::{TEXTURE_ID_MIKU_WAVE, sprite_info_miku_wave};
use crate::game::GameContext;
use crate::grid_layout::GridLayout;
use crate::renderer::RenderCommand;
use crate::{ReadyState, advance_ready_state};

pub struct ShuttingDownScene {
    miku: SpriteInfo,
    countdown: ReadyState,
}

impl ShuttingDownScene {
    fn layout(game_context: &GameContext) -> GridLayout {
        let (screen_width, screen_height) = game_context.screen_size;
        GridLayout {
            area: Rect::new(0, 0, screen_width as isize, screen_height as isize),
            rows: 3,
            columns: 3,
            cell_gap: 0,
        }
    }
}

impl Default for ShuttingDownScene {
    fn default() -> Self {
        Self {
            miku: sprite_info_miku_wave(),
            countdown: ReadyState::Cooldown {
                ticks_waited: 0,
                wait_for: 120,
            },
        }
    }
}

impl Scene for ShuttingDownScene {
    fn init(&mut self, game_context: &mut GameContext) {
        let Some(ref mut asset_loader) = game_context.asset_loader else {
            return;
        };
        asset_loader.ensure_texture_spritesheet_loaded(TEXTURE_ID_MIKU_WAVE);
    }
    fn update(&mut self, ticks: u32, game_context: &mut GameContext) {
        self.miku.advance(ticks);
        self.countdown = advance_ready_state(self.countdown, ticks);
        let ReadyState::Ready = self.countdown else {
            return;
        };
        game_context.shutdown_flag = true;
    }
    fn draw(&mut self, game_context: &mut GameContext) {
        let layout = ShuttingDownScene::layout(game_context);
        let Some(ref mut renderer) = game_context.renderer else {
            return;
        };

        let destination = layout.cell_rect(1, 1);
        let src = self.miku.get_rect();
        renderer.send_command(RenderCommand::DrawRect {
            texture_id: TEXTURE_ID_MIKU_WAVE,
            source: src,
            destination,
        });
    }
}
