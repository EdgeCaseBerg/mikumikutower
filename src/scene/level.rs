use crate::Rect;
use crate::Scene;
use crate::SpriteInfo;
use crate::constants::{TEXTURE_ID_LEEKSHEET, TEXTURE_ID_MIKU, TEXTURE_ID_PORTRAIT};
use crate::game::GameContext;
use crate::grid_layout::GridLayout;
use crate::renderer::RenderCommand;

#[derive(Debug, Clone)]
enum TowerState {
    Ready,
    Cooldown { wait_for: u8, ticks_waited: u8 },
}

#[derive(Debug, Clone)]
struct Health {
    current: u8,
    max: u8,
}

impl Health {
    fn damage(&mut self, amount: u8) {
        self.current = self.current.saturating_sub(amount);
    }
}

impl Default for Health {
    fn default() -> Health {
        let max = 10;
        Health { current: max, max }
    }
}

#[derive(Debug)]
struct Base {
    position: Rect,
    health: Health,
    sprite_info: SpriteInfo, // a miku sprite
}

impl Default for Base {
    fn default() -> Base {
        Base {
            position: Rect::new(1, 2, 64, 64),
            health: Health::default(),
            sprite_info: SpriteInfo {
                // corresponds to TEXTURE_ID_MIKU (should we do a sprite for constant helper?)
                start_x: 0,
                start_y: 0,
                width: 71,
                height: 54,
                frames: 6,
                current_frame: 0,
                framerate_per_second: 10,
                delta: 0,
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Tower {
    position: Rect,
    state: TowerState,
    range: u16, // 65535 should be enough
    // Add types later or some such thing.
    sprite_info: SpriteInfo, // a leek sprite for now
}

impl Tower {
    fn basic(position: Rect) -> Self {
        Self {
            position,
            state: TowerState::Ready,
            range: 5, // TODO: revisit once we decide how big our gameboard is
            sprite_info: SpriteInfo {
                start_x: 0,
                start_y: 0,
                width: 32,
                height: 32,
                frames: 1,
                current_frame: 0,
                framerate_per_second: 60,
                delta: 0,
            },
        }
    }
}

pub struct LevelScene {
    base: Base,
    towers: Vec<Tower>,
    grass: SpriteInfo,
    road: SpriteInfo,
}

// TODO: both base and rect right now are x,y in world coordinates w,h in screen. we should fix that up.
impl Default for LevelScene {
    fn default() -> LevelScene {
        let initial_towers = vec![Tower::basic(Rect::new(16, 16, 32, 32))];
        LevelScene {
            base: Base::default(),
            towers: initial_towers,
            grass: SpriteInfo {
                start_x: 64,
                start_y: 0,
                width: 32,
                height: 32,
                frames: 1,
                current_frame: 0,
                framerate_per_second: 60,
                delta: 0,
            },
            road: SpriteInfo {
                start_x: 32,
                start_y: 0,
                width: 32,
                height: 32,
                frames: 1,
                current_frame: 0,
                framerate_per_second: 60,
                delta: 0,
            },
        }
    }
}

impl Scene for LevelScene {
    fn init(&mut self, game_context: &mut GameContext) {}

    fn update(&mut self, ticks: u32, game_context: &mut GameContext) {
        self.grass.advance(ticks);
        self.road.advance(ticks);
        self.base.sprite_info.advance(ticks);
    }

    fn draw(&mut self, game_context: &mut GameContext) {
        let Some(ref mut renderer) = game_context.renderer else {
            return;
        };

        let layout = GridLayout {
            area: Rect::new(0, 0, 1280, 720),
            rows: 18,
            columns: 32,
            cell_gap: 0,
        };
        for (r, c, cell) in layout.iter_cells() {
            let src = match (r, c) {
                (16, c) if c > 3 && c < 28 => self.road.get_rect(),
                (r, 27) if r >= 0 && r < 16 => self.road.get_rect(),
                _ => self.grass.get_rect(),
            };
            renderer.send_command(RenderCommand::DrawRect {
                texture_id: TEXTURE_ID_LEEKSHEET,
                source: src,
                destination: cell,
            });
            if r == 16 && c == 3 {
                let src = self.base.sprite_info.get_rect();
                renderer.send_command(RenderCommand::DrawRect {
                    texture_id: TEXTURE_ID_MIKU,
                    source: src,
                    destination: cell,
                });
            }
        }
    }
}
