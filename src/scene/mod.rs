use crate::SpriteInfo;
use crate::game::Game;
use crate::game::GameContext;
use crate::renderer::Rect;
use crate::renderer::RenderCommand;

pub mod loading_scene;

pub trait Scene {
    fn init(&mut self, game_context: &mut GameContext) {}
    fn update(&mut self, ticks: u32, game_context: &mut GameContext) {}
    fn draw(&mut self, game_context: &mut GameContext) {}
}
