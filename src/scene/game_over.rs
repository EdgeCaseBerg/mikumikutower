use crate::Rect;
use crate::Scene;
use crate::SpriteInfo;
use crate::constants::{
    TEXTURE_ID_FONTSHEET, TEXTURE_ID_GAMEOVER, TEXTURE_ID_LEEKSHEET, sprite_info_gameover_miku,
    sprite_info_topbar_bg,
};
use crate::font::{center_font_in_tile, get_rects_for_str};
use crate::game::GameContext;
use crate::grid_layout::GridLayout;
use crate::renderer::RenderCommand;

struct Button {
    text: String,
    rect: Rect,
    hovered: bool,
    clicked: bool,
    bg: SpriteInfo,
}

impl Button {
    fn new(text: String, rect: Rect) -> Self {
        Self {
            text,
            rect,
            hovered: false,
            clicked: false,
            bg: sprite_info_topbar_bg(),
        }
    }

    fn relative_layout(&self, parent_layout: &GridLayout) -> GridLayout {
        let anchor = parent_layout.cell_rect(self.rect.y as usize, self.rect.x as usize);
        let width = anchor.width * self.rect.width;
        let height = anchor.height * self.rect.height;
        GridLayout {
            area: Rect {
                x: anchor.x,
                y: anchor.y,
                width,
                height,
            },
            rows: 1,
            columns: 1,
            cell_gap: 0,
        }
    }

    fn update(&mut self, game_context: &GameContext, parent_layout: &GridLayout) {
        let layout = self.relative_layout(parent_layout);
        let Some((r, c, cell)) = layout.cell_for_mouse(game_context.mouse_context.position) else {
            self.hovered = false;
            self.clicked = false;
            return;
        };

        self.hovered = true;

        if game_context.mouse_context.left_clicked {
            self.clicked = true;
        }
    }

    fn draw(&mut self, game_context: &mut GameContext, parent_layout: &GridLayout) {
        let Some(ref mut renderer) = game_context.renderer else {
            return;
        };
        let layout = self.relative_layout(parent_layout);
        let mut col = 2;
        let src = self.bg.get_rect();
        let anchor = layout.cell_rect(0, 0);
        renderer.send_command(RenderCommand::DrawRect {
            texture_id: TEXTURE_ID_LEEKSHEET,
            source: src,
            destination: anchor,
        });

        let glyphs = get_rects_for_str(&self.text);
        let (cx, cy) = anchor.center();
        for (c, src) in glyphs.iter().enumerate() {
            // let mut cell = center_font_in_tile(anchor, *src, c as isize);
            let glyph_display_width = anchor.width / (glyphs.len() as isize + 2);
            // mono font is same height as width (16x16 native)
            let glyph_display_height = glyph_display_width;
            let start_offset = anchor.x + glyph_display_width;
            let cell = Rect {
                x: start_offset + c as isize * glyph_display_width as isize,
                y: cy - glyph_display_height as isize / 2,
                width: glyph_display_width,
                height: glyph_display_height,
            };
            renderer.send_command(RenderCommand::DrawRect {
                texture_id: TEXTURE_ID_FONTSHEET,
                source: *src,
                destination: cell,
            });
        }
    }
}

pub struct GameOverScene {
    // TODO: probably move sprite info around or something... but for now, test scene!
    miku: SpriteInfo,
    try_again_btn: Button,
    give_up_btn: Button,
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
        }
    }
}

impl Scene for GameOverScene {
    fn init(&mut self, _game_context: &mut GameContext) {}

    fn update(&mut self, ticks: u32, game_context: &mut GameContext) {
        let layout = GameOverScene::layout(&game_context);
        self.give_up_btn.update(game_context, &layout);
        self.try_again_btn.update(game_context, &layout);

        // Check where mouse is, hover over quit -> miku sobbing (frame 1)
        if self.give_up_btn.hovered {
            self.miku.current_frame = 1;
        } else if self.try_again_btn.hovered {
            self.miku.current_frame = 2;
        } else {
            self.miku.current_frame = 0;
        }

        // Check where mouse is, hover over try again -> miku getting up (frame 2)
        // no mouse over button -> miku on ground (frame 0)
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
