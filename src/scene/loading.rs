use crate::Rect;
use crate::Scene;
use crate::SpriteInfo;
use crate::constants::{
    TEXTURE_ID_MIKU, TEXTURE_ID_PORTRAIT, sprite_info_miku, sprite_info_portrait,
};
use crate::game::GameContext;
use crate::renderer::RenderCommand;

pub struct TestScene {
    // TODO: probably move sprite info around or something... but for now, test scene!
    sprites: Vec<(usize, SpriteInfo)>,
}

impl Default for TestScene {
    fn default() -> TestScene {
        TestScene {
            sprites: Vec::new(),
        }
    }
}

impl Scene for TestScene {
    fn init(&mut self, game_context: &mut GameContext) {
        let miku = sprite_info_miku();
        let portrait = sprite_info_portrait();

        self.sprites.push((TEXTURE_ID_MIKU, miku));
        self.sprites.push((TEXTURE_ID_PORTRAIT, portrait));

        let Some(ref mut asset_loader) = game_context.asset_loader else {
            return;
        };
        asset_loader.ensure_texture_spritesheet_loaded(TEXTURE_ID_MIKU);
        asset_loader.ensure_texture_spritesheet_loaded(TEXTURE_ID_PORTRAIT);
    }
    fn update(&mut self, ticks: u32, _game_context: &mut GameContext) {
        for (_, sprite) in self.sprites.iter_mut() {
            sprite.advance(ticks);
        }
    }
    fn draw(&mut self, game_context: &mut GameContext) {
        let Some(ref mut renderer) = game_context.renderer else {
            return;
        };

        for (id, sprite) in self.sprites.iter() {
            let (x, y) = match id {
                &TEXTURE_ID_MIKU => (200, 600),
                _ => (0, 0),
            };
            let src = sprite.get_rect();
            renderer.send_command(RenderCommand::DrawRect {
                texture_id: *id,
                source: src,
                destination: Rect::new(x, y, src.width, src.height),
            });
        }
    }
}
