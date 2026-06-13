use mikumikutower::game::Game;
use mikumikutower::game_options::GameOptions;

fn main() {
    let options = GameOptions::default();
    let game = Game::new();
    mikumikutower::run(&options, game);
}

// Note if you looking for the entry point for wasm, it's not here, it's over in lib.rs See "start"
