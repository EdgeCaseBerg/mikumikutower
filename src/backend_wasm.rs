use crate::asset_loader::AssetLoader;
use crate::audio::{Audio, AudioResult};
use crate::backend::*;
use crate::constants::*;
use crate::constants::{MusicId, SfxId};
use crate::game::Game;
use crate::game::GameContext;
use crate::game_options::GameOptions;
use crate::renderer::{Color, RenderCommand, Renderer};

use std::time::Duration;

pub struct BackendWasm {}

pub struct WasmSounds {}

impl WasmSounds {
    fn new(_game_options: &GameOptions) -> Self {
        WasmSounds {}
    }
}

impl Audio for WasmSounds {
    fn play_sfx(&mut self, _id: SfxId) -> AudioResult<()> {
        Ok(())
    }
    fn load_sfx(&mut self, _sound_id: SfxId) -> AudioResult<()> {
        Ok(())
    }
    fn play_music(&mut self, _id: MusicId) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    /// Calling this method with the same id multiple times will only load the music once.
    fn load_music(&mut self, _id: MusicId) -> AudioResult<()> {
        Ok(())
    }
    fn load_bg_music(&mut self) -> Vec<AudioResult<MusicId>> {
        let ids = Vec::new();
        ids
    }
    fn music_duration_seconds(&self, _id: MusicId) -> AudioResult<Duration> {
        Ok(Duration::from_millis(0))
    }

    fn prepare(&mut self) -> Vec<AudioResult<()>> {
        let stream_failures = vec![];
        stream_failures
    }
}

struct AssetLoaderWasm {}

impl AssetLoader for AssetLoaderWasm {
    fn ensure_texture_spritesheet_loaded(&mut self, _id: TextureId) {}
}

impl BackendWasm {
    pub fn new(_game_options: &GameOptions) -> Self {
        BackendWasm {}
    }
}

impl Backend for BackendWasm {
    fn create_event_loop(&self, _game_options: &GameOptions) -> Box<dyn BackendEventLoop> {
        let e = EventLoopWasm {};
        Box::new(e)
    }
}

pub struct EventLoopWasm {}

impl BackendEventLoop for EventLoopWasm {
    fn run(&mut self, game: &mut Game, game_context: &mut GameContext) {
        // Look it's wasm!
        web_sys::console::log_1(&"hello wasm".into());

        let scene = game.scene.as_mut();
        if let Some(scene) = scene {
            scene.init(game_context);
        }

        // initialize the audio pool if the scene has queued things up
        let audio = game_context.audio.as_mut();
        if let Some(audio) = audio {
            let _ = audio.prepare();
        }

        // TODO: this is where we need to do the closure callback dance.
        game.update(game_context);
        if let Some(mut next_scene) = game_context.next_scene.take() {
            next_scene.init(game_context);
            game.scene = Some(next_scene);
            game.reset_for_next_scene();
            let audio = game_context.audio.as_mut();
            if let Some(audio) = audio {
                audio.prepare();
            }
        }
        game.draw(game_context);
        if game_context.shutdown_flag {
            // break; TODO restore break or something?
            return;
        }
    }

    fn new_renderer(&self, _game_options: &GameOptions) -> Box<dyn Renderer> {
        let r = RendererWasm { commands: vec![] };
        Box::new(r)
    }

    fn create_asset_loader(&self, _game_options: &GameOptions) -> Box<dyn AssetLoader> {
        let a = AssetLoaderWasm {};
        Box::new(a)
    }

    fn create_audio(&self, game_options: &GameOptions) -> Box<dyn Audio> {
        let s = WasmSounds::new(game_options);
        Box::new(s)
    }
}

struct RendererWasm {
    commands: Vec<RenderCommand>,
}

impl RendererWasm {
    // Internally used before presenting. Drains all commands
    // in order to enque all the work to SDL3 that we want done
    // per frame.
    fn process_commands(&mut self) {
        for cmd in self.commands.drain(..) {
            match cmd {
                RenderCommand::DrawRect {
                    texture_id: _,
                    source: _,
                    destination: _,
                } => {
                    // TODO do this sort of thing but with wasm.
                    // let ctx = &mut *self.context.borrow_mut();
                    // if let Some(texture) = ctx.textures.get_texture(texture_id) {
                    //     let src: sdl3::rect::Rect = source.into();
                    //     let dst: sdl3::rect::Rect = destination.into();
                    //     ctx.window_canvas
                    //         .copy(texture, src, dst)
                    //         .unwrap_or_else(|_| {
                    //             let _ = &format!("failed to draw texture {}", texture_id.0);
                    //         });
                    // }
                }
            }
        }
    }
}

impl Renderer for RendererWasm {
    fn name(&self) -> String {
        "WASM Renderer".to_string()
    }

    fn clear(&mut self, _color: Color) {}

    fn present(&mut self) {
        self.process_commands();
    }

    fn send_command(&mut self, cmd: RenderCommand) {
        self.commands.push(cmd);
    }
}
