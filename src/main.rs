use mikumikutower::game::Game;
use mikumikutower::game_options::GameOptions;

fn main() {
    let options = GameOptions::default();
    let mut game = Game::new();
    mikumikutower::hello_sdl(&options, &mut game);
}
