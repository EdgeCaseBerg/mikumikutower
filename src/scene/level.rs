use crate::Rect;
use crate::Scene;
use crate::SpriteInfo;
use crate::constants::{
    TEXTURE_ID_LEEKSHEET, TEXTURE_ID_MIKU, sprite_info_grass, sprite_info_leek, sprite_info_miku,
    sprite_info_road,
};
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
            position: Rect::new(3, 16, 32, 32),
            health: Health::default(),
            sprite_info: sprite_info_miku(),
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
            sprite_info: sprite_info_leek(),
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
        let initial_towers = vec![Tower::basic(Rect::new(26, 15, 32, 32))];
        LevelScene {
            base: Base::default(),
            towers: initial_towers,
            grass: sprite_info_grass(),
            road: sprite_info_road(),
        }
    }
}

impl Scene for LevelScene {
    fn init(&mut self, game_context: &mut GameContext) {}

    fn update(&mut self, ticks: u32, game_context: &mut GameContext) {
        self.grass.advance(ticks);
        self.road.advance(ticks);
        self.base.sprite_info.advance(ticks);
        for mut tower in &mut self.towers {
            tower.sprite_info.advance(ticks);
        }
    }

    fn draw(&mut self, game_context: &mut GameContext) {
        let Some(ref mut renderer) = game_context.renderer else {
            return;
        };
        let (screen_width, screen_height) = game_context.screen_size;

        let layout = GridLayout {
            area: Rect::new(0, 0, screen_width as isize, screen_height as isize),
            rows: 18,
            columns: 32,
            cell_gap: 0,
        };
        for (r, c, cell) in layout.iter_cells() {
            let src = match (r, c) {
                (16, c) if c > 3 && c < 28 => self.road.get_rect(),
                (r, 27) if r < 16 => self.road.get_rect(),
                _ => self.grass.get_rect(),
            };
            renderer.send_command(RenderCommand::DrawRect {
                texture_id: TEXTURE_ID_LEEKSHEET,
                source: src,
                destination: cell,
            });
        }
        let cell = layout.cell_rect(self.base.position.y as usize, self.base.position.x as usize);
        let src = self.base.sprite_info.get_rect();
        renderer.send_command(RenderCommand::DrawRect {
            texture_id: TEXTURE_ID_MIKU,
            source: src,
            destination: cell,
        });

        for tower in self.towers.iter() {
            let cell = layout.cell_rect(tower.position.y as usize, tower.position.x as usize);
            let src = tower.sprite_info.get_rect();
            renderer.send_command(RenderCommand::DrawRect {
                texture_id: TEXTURE_ID_LEEKSHEET,
                source: src,
                destination: cell,
            });
        }

        // for testing we'll just use the leak
        if let Some((r, c, cell)) = layout.cell_for_mouse(game_context.mouse_context.position) {
            let src = sprite_info_leek().get_rect();
            renderer.send_command(RenderCommand::DrawRect {
                texture_id: TEXTURE_ID_LEEKSHEET,
                source: src,
                destination: cell,
            });
        }
    }
}
