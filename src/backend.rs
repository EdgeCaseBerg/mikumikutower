use crate::asset_loader::AssetLoader;
use crate::audio::Audio;
use crate::game::Game;
use crate::game::GameContext;
use crate::game_options::GameOptions;
use crate::renderer::Renderer;

pub trait Backend {
    fn create_event_loop(&self, game_options: &GameOptions) -> Box<dyn BackendEventLoop>;
}

pub trait BackendEventLoop {
    fn run(&mut self, game: &mut Game, game_context: &mut GameContext);
    fn new_renderer(&self, game_options: &GameOptions) -> Box<dyn Renderer>;
    fn create_asset_loader(&self, game_options: &GameOptions) -> Box<dyn AssetLoader>;
    fn create_audio(&self, game_options: &GameOptions) -> Box<dyn Audio>;
}

pub fn init_backend(game_options: &GameOptions) -> Box<dyn Backend> {
    // There is only one backend to init right now but this is where we could
    // do fun #if(config) type things in the future if need be!
    #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
    {
        use crate::backend_sdl3::BackendSDL3;
        Box::new(BackendSDL3::new(game_options))
    }

    #[cfg(all(target_family = "wasm", target_os = "unknown"))]
    {
        use crate::backend_wasm::BackendWasm;
        Box::new(BackendWasm::new(game_options))
    }
}
