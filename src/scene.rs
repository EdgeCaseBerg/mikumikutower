use crate::SpriteInfo;
use crate::game::Game;
use crate::game::GameContext;
use crate::renderer::Rect;
use crate::renderer::RenderCommand;

pub trait Scene {
    fn init(&mut self, game_context: &mut GameContext) {}
    fn update(&mut self, ticks: u32, game_context: &mut GameContext) {}
    fn draw(&mut self, game_context: &mut GameContext) {}
}

// TODO: move scene and test to own file

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

        // TODO: we'll move the texture ids out to constants to match up with the backend renderer load calls in renderer
        let miku = SpriteInfo {
            start_x: 0,
            start_y: 0,
            width: 71,
            height: 54,
            frames: 6,
            current_frame: 0,
            framerate_per_second: 10,
            delta: 0,
        };
        self.sprites.push((0, miku));

        let portrait = SpriteInfo {
            start_x: 0,
            start_y: 0,
            width: 2478,
            height: 402,
            frames: 1,
            current_frame: 0,
            framerate_per_second: 60,
            delta: 0,
        };
        self.sprites.push((1, portrait));

        // TODO: call load here instead maybe?
        // if Some(renderer) = game_context.renderer {

        // }
    }
    fn update(&mut self, ticks: u32, game_context: &mut GameContext) {
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
                0 => (200, 600),
                _ => (0, 0)
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
