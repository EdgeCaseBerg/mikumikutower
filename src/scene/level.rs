use crate::Rect;
use crate::Scene;
use crate::SpriteInfo;
use crate::constants::{
    TEXTURE_ID_LEEKSHEET, TEXTURE_ID_MIKU, sprite_info_grass, sprite_info_highlight,
    sprite_info_leek, sprite_info_luka_tower, sprite_info_miku, sprite_info_miku_tower,
    sprite_info_rin_tower, sprite_info_road,
};
use crate::game::GameContext;
use crate::grid_layout::GridLayout;
use crate::renderer::RenderCommand;

#[derive(Debug, Clone)]
enum TowerState {
    Ready,
    Cooldown { wait_for: u32, ticks_waited: u32 },
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
    cost: u32,
    damage: u32,
    cooldown: u32,
}

impl Tower {
    fn update(&mut self, ticks: u32) {
        match self.state {
            TowerState::Ready => {
                // todo: if we had a target to fire at then we'd fire and transition
                // into the cooldown state
            },
            TowerState::Cooldown { wait_for, ticks_waited } => {
                let ticks_waited = ticks_waited.saturating_add(ticks);
                if ticks_waited >= wait_for {
                    self.state = TowerState::Ready;
                } else {
                    self.state = TowerState::Cooldown {
                        wait_for, ticks_waited
                    };
                }
            }
        }
    }

    fn basic(position: Rect) -> Self {
        Self {
            position,
            state: TowerState::Ready,
            range: 5, // TODO: revisit once we decide how big our gameboard is
            sprite_info: sprite_info_leek(),
            cost: 10,
            damage: 1,
            cooldown: 30,
        }
    }

    fn miku(position: Rect) -> Self {
        let mut base = Self::basic(position);
        base.sprite_info = sprite_info_miku_tower();
        base.cost = 20;
        base.damage = 5;
        base.cooldown = 30;
        base
    }

    fn rin(position: Rect) -> Self {
        let mut base = Self::basic(position);
        base.sprite_info = sprite_info_rin_tower();
        base.cost = 15;
        base.damage = 3;
        base.cooldown = 15;
        base
    }

    fn luka(position: Rect) -> Self {
        let mut base = Self::basic(position);
        base.sprite_info = sprite_info_luka_tower();
        base.cost = 30;
        base.damage = 15;
        base.cooldown = 60;
        base
    }
}

#[derive(Debug, Clone)]
enum PlayerAction {
    PlaceTower(Tower),
    // more later as desired.
}

pub struct TopBar {
    miku_tower: Tower, // average useful tower
    rin_tower: Tower,  // speedy but less damage
    luka_tower: Tower, // slow but strong
    current_action: Option<PlayerAction>,
    money: u32,
}

impl Default for TopBar {
    fn default() -> TopBar {
        TopBar {
            miku_tower: Tower::miku(Rect::new(0, 0, 32, 32)),
            rin_tower: Tower::rin(Rect::new(1, 0, 32, 32)),
            luka_tower: Tower::luka(Rect::new(2, 0, 32, 32)),
            current_action: None,
            money: 50, // TODO: figure out a good starting point for this
        }
    }
}

impl TopBar {
    fn update(&mut self, ticks: u32, game_context: &mut GameContext, layout: &GridLayout) {
        let Some((r, c, rect)) = layout.cell_for_mouse(game_context.mouse_context.position) else {
            return;
        };

        for tower in vec![
            &mut self.miku_tower,
            &mut self.rin_tower,
            &mut self.luka_tower,
        ] {
            let tower_cell = layout.cell_rect(tower.position.y as usize, tower.position.x as usize);
            if tower_cell.contains(rect.x + 1, rect.y + 1) {
                tower.sprite_info.advance(ticks);
                if game_context.mouse_context.left_clicked
                    && self.current_action.is_none()
                    && self.money >= tower.cost
                {
                    self.current_action = Some(PlayerAction::PlaceTower(tower.clone()));
                    game_context.mouse_context.consume_left_click();
                    // annoyingly, we cant call self.buy_tower without the borrow checker bitching.
                    // because it can't understand that only the money field will be mutated within that call.
                    self.money = self.money.saturating_sub(tower.cost);
                    eprintln!("Buy tower {}", self.money);
                }
            }
        }

        if game_context.mouse_context.right_clicked {
            match &self.current_action {
                Some(PlayerAction::PlaceTower(tower)) => {
                    self.money = self.money.saturating_add(tower.cost);
                    game_context.mouse_context.consume_right_click();
                    self.current_action = None;
                }
                _ => {}
            }
        }
    }

    fn draw(&mut self, game_context: &mut GameContext, layout: &GridLayout) {
        let Some(ref mut renderer) = game_context.renderer else {
            return;
        };

        for tower in vec![&self.miku_tower, &self.rin_tower, &self.luka_tower] {
            let cell = layout.cell_rect(tower.position.y as usize, tower.position.x as usize);
            let src = tower.sprite_info.get_rect();
            renderer.send_command(RenderCommand::DrawRect {
                texture_id: TEXTURE_ID_LEEKSHEET,
                source: src,
                destination: cell,
            });
        }
    }

    fn buy_tower(&mut self, tower: &Tower) {
        if tower.cost > self.money {
            eprintln!("Dont Buy tower {}", self.money);
            return;
        }
        self.money = self.money.saturating_sub(tower.cost);
        eprintln!("Buy tower {}", self.money);
    }

    fn refund_tower(&mut self, tower: &Tower) {
        self.money = self.money.saturating_add(tower.cost);
        eprintln!("Refund tower {}", self.money);
    }
}

pub struct LevelScene {
    base: Base,
    towers: Vec<Tower>,
    grass: SpriteInfo,
    road: SpriteInfo,
    highlight: SpriteInfo,
    top_bar: TopBar,
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
            highlight: sprite_info_highlight(),
            top_bar: TopBar::default(),
        }
    }
}

impl LevelScene {
    fn check_action(&mut self, layout: &GridLayout, game_context: &mut GameContext) {
        let Some(PlayerAction::PlaceTower(tower_to_place)) = &self.top_bar.current_action else {
            return;
        };

        let Some((r, c, cell)) = layout.cell_for_mouse(game_context.mouse_context.position) else {
            return;
        };

        let legal_placement = {
            let not_in_menu = r > 0;
            let not_on_other_tower = !self
                .towers
                .iter()
                .any(|t| t.position.x == c as isize && t.position.y == r as isize);
            not_in_menu && not_on_other_tower
        };
        if game_context.mouse_context.left_clicked && legal_placement {
            let action = self.top_bar.current_action.take();
            let Some(PlayerAction::PlaceTower(mut tower)) = action else {
                unreachable!();
            };
            tower.position.x = c as isize;
            tower.position.y = r as isize;
            self.towers.push(tower);
            game_context.mouse_context.consume_left_click();
        }

        // calling top_bar update should mean we dont need to do this. but doesnt hurt to be sure.
        if game_context.mouse_context.right_clicked {
            let action = self.top_bar.current_action.take();
            let Some(PlayerAction::PlaceTower(mut tower)) = action else {
                unreachable!();
            };
            self.top_bar.refund_tower(&tower);
            game_context.mouse_context.consume_right_click();
        }
    }
}

impl Scene for LevelScene {
    fn init(&mut self, game_context: &mut GameContext) {}

    fn update(&mut self, ticks: u32, game_context: &mut GameContext) {
        let (screen_width, screen_height) = game_context.screen_size;
        let layout = GridLayout {
            area: Rect::new(0, 0, screen_width as isize, screen_height as isize),
            rows: 18,
            columns: 32,
            cell_gap: 0,
        };

        self.grass.advance(ticks);
        self.road.advance(ticks);
        self.base.sprite_info.advance(ticks);
        self.highlight.advance(ticks);
        for mut tower in &mut self.towers {
            tower.sprite_info.advance(ticks);
        }
        self.top_bar.update(ticks, game_context, &layout);
        self.check_action(&layout, game_context);
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

        if let Some((r, c, cell)) = layout.cell_for_mouse(game_context.mouse_context.position) {
            if let Some(PlayerAction::PlaceTower(tower_to_place)) = &self.top_bar.current_action {
                let src = tower_to_place.sprite_info.get_rect();
                renderer.send_command(RenderCommand::DrawRect {
                    texture_id: TEXTURE_ID_LEEKSHEET,
                    source: src,
                    destination: cell,
                });
            }

            let src = self.highlight.get_rect();
            renderer.send_command(RenderCommand::DrawRect {
                texture_id: TEXTURE_ID_LEEKSHEET,
                source: src,
                destination: cell,
            });
        }
        self.top_bar.draw(game_context, &layout);
    }
}
