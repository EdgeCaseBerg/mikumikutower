use crate::game::GameContext;

pub mod game_over;
pub mod level;
pub mod loading;

pub trait Scene {
    fn init(&mut self, game_context: &mut GameContext);
    fn update(&mut self, ticks: u32, game_context: &mut GameContext);
    fn draw(&mut self, game_context: &mut GameContext);
}
