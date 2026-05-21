use crate::Rect;
use crate::Scene;
use crate::SpriteInfo;
use crate::constants::{
    SFX_ID_BLIP, TEXTURE_ID_FONTSHEET, TEXTURE_ID_LEEKSHEET, TEXTURE_ID_TITLE_BG,
    sprite_info_highlight, sprite_info_title, sprite_info_topbar_bg,
};
use crate::font::get_rects_for_str;
use crate::game::GameContext;
use crate::grid_layout::GridLayout;
use crate::renderer::RenderCommand;

//todo: share this I suppose and move it to mod or something
struct Button {
    text: String,
    rect: Rect,
    hovered: bool,
    clicked: bool,
    bg: SpriteInfo,
    highlight: SpriteInfo,
}

impl Button {
    fn new(text: String, rect: Rect) -> Self {
        Self {
            text,
            rect,
            hovered: false,
            clicked: false,
            bg: sprite_info_topbar_bg(),
            highlight: sprite_info_highlight(),
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

    fn update(&mut self, ticks: u32, game_context: &GameContext, parent_layout: &GridLayout) {
        let layout = self.relative_layout(parent_layout);
        let Some(_) = layout.cell_for_mouse(game_context.mouse_context.position) else {
            self.hovered = false;
            self.clicked = false;
            return;
        };

        self.hovered = true;
        self.highlight.advance(ticks);

        if game_context.mouse_context.left_clicked {
            self.clicked = true;
        }
    }

    fn draw(&mut self, game_context: &mut GameContext, parent_layout: &GridLayout) {
        let Some(ref mut renderer) = game_context.renderer else {
            return;
        };
        let layout = self.relative_layout(parent_layout);
        let src = self.bg.get_rect();
        let anchor = layout.cell_rect(0, 0);
        renderer.send_command(RenderCommand::DrawRect {
            texture_id: TEXTURE_ID_LEEKSHEET,
            source: src,
            destination: anchor,
        });

        if self.hovered {
            let src = self.highlight.get_rect();
            renderer.send_command(RenderCommand::DrawRect {
                texture_id: TEXTURE_ID_LEEKSHEET,
                source: src,
                destination: anchor,
            });
        }

        let glyphs = get_rects_for_str(&self.text);
        let (_, cy) = anchor.center();
        for (c, src) in glyphs.iter().enumerate() {
            // leave room for a glpyh on either side
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

pub struct TitleScene {
    bg: SpriteInfo,
    start_game_btn: Button,
    quit_btn: Button,
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
    }
    fn update(&mut self, ticks: u32, game_context: &mut GameContext) {
        let layout = TitleScene::layout(&game_context);
        self.quit_btn.update(ticks, game_context, &layout);
        self.start_game_btn.update(ticks, game_context, &layout);

        if self.start_game_btn.clicked && game_context.next_scene.is_none() {
            game_context.audio.as_mut().map(|audio| {
                audio.play_sfx(SFX_ID_BLIP);
            });
            game_context.queue_level();
            self.start_game_btn.clicked = false;
        }

        if self.quit_btn.clicked && game_context.next_scene.is_none() {
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
