use crate::asset_loader::AssetLoader;
use crate::audio::{Audio, AudioResult};
use crate::backend::*;
use crate::clock::Clock;
use crate::constants::*;
use crate::constants::{MusicId, SfxId};
use crate::game::Game;
use crate::game::GameContext;
use crate::game_options::GameOptions;
use crate::renderer::{Color, RenderCommand, Renderer};

use std::cell::RefCell;
use std::rc::Rc;

use std::time::Duration; // this is probably going to bite us later.

use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{Document, HtmlCanvasElement, HtmlElement, Window};

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

struct WasmClock {
    start: u128,
}

fn milli_to_nano(milliseconds: f64) -> u128 {
    (milliseconds * 1_000_000.0) as u128
}

impl WasmClock {
    fn new() -> Self {
        let window = web_sys::window().expect("no browser window found");
        let milliseconds = window
            .performance()
            .expect("no performance in browser defined")
            .now();
        let nanos = milli_to_nano(milliseconds);
        WasmClock { start: nanos }
    }
}

impl Clock for WasmClock {
    fn elapsed_since_start(&self) -> u128 {
        let window = web_sys::window().expect("no browser window found");
        let now = window
            .performance()
            .expect("no performance in browser defined")
            .now();
        let now = milli_to_nano(now);
        let nanos = now - self.start;
        nanos
    }
}

/////////////////////////////////////////////////////////////////////
// Helpers shameless lifted out of wasm-bingen.github.io
//
//
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣿⣿⣷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣾⣿⣿⣿⣿⣿⣷⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⣴⣿⣷⣶⣦⣤⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣾⣿⢟⣯⣭⣝⠻⣿⣿⣷⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⢀⡿⠟⠿⢿⣿⣿⣿⣿⣷⣄⠀⠀⠀⠀⠀⣰⣿⣿⢣⣿⠟⠀⠈⢻⡘⣿⣿⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⢸⡇⣟⠛⠓⠮⣝⣻⣿⡿⠟⠛⠛⠛⠛⠛⣿⣿⣏⣿⣟⣀⠀⠀⠀⣷⢹⣿⣿⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⢸⣇⢿⡀⠀⣠⠞⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠉⠛⠶⢦⣽⣸⣿⣿⣷⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠸⣿⣜⣷⠞⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠛⣿⣿⣿⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⡿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⢧⡀⠀⠀⠀⠹⣿⣿⣿⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⣾⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢳⡄⠀⠀⠀⢻⣿⣿⣿⣷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⢰⡇⠀⠀⠀⠀⢰⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⡄⠀⠀⢸⣿⣿⡿⣫⣿⣧⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⢸⠁⠀⠀⠀⠀⢸⠋⢧⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠹⡄⠀⠸⣿⣿⠡⣿⣿⣿⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀It's⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⢸⡄⠀⠀⢀⠀⡾⠀⠀⠑⢦⡀⠀⠀⠀⠀⠀⠀⣀⡤⠀⠀⠀⠀⢀⡀⠀⠀⠙⣆⠀⠹⣿⣷⣝⠿⣿⣿⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀mine⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠘⣇⠀⠀⢸⣿⠃⣀⡀⠀⠀⠙⢆⠀⠀⠀⠀⠚⣩⣀⠀⠀⠀⢀⣘⣿⠶⣄⣠⡽⠄⠀⠹⣿⣿⣷⣶⠋⠻⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀now!⠀⠀⠀⠀⠀
//⠀⠀⠀⠘⡀⠀⠈⡟⠀⠀⠙⠦⠀⠀⠈⠛⢦⣀⣀⣠⣽⠍⠳⠒⠛⣭⣵⠆⠀⡇⠀⠀⠀⠀⠀⣿⣿⡿⠃⠀⠀⠸⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⣇⠐⠦⣷⣤⣤⣀⡀⠀⠀⠀⠀⠀⠁⠀⠀⢀⣀⣀⣴⣿⠟⠁⠀⠀⡇⠀⠀⠀⠀⠀⢸⠋⠀⠀⠀⠀⣠⠟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⣹⠤⠶⠋⠉⠙⠛⠛⠷⠄⠀⠀⠀⠀⠀⠀⠀⠘⠛⠉⠀⠀⠀⠀⠀⠧⣴⣶⣶⣤⣤⣿⡆⣀⢾⠥⠞⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⢧⡀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⡦⣄⣀⣰⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⠟⠻⢯⡿⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠻⣿⣳⣦⣄⣀⠀⠀⠀⠀⠀⠘⣧⡤⠶⠿⠦⣤⣤⠴⠖⠛⠦⣤⠴⣾⠁⠀⠀⠀⠀⣆⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠙⠻⡏⠀⠈⠉⢻⡗⢲⠒⡶⠁⠀⠀⠀⠀⠀⢿⠳⣦⠀⠀⠈⢳⣿⣦⣤⣀⣠⣤⠼⢿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠲⠧⣤⣤⣤⣾⡯⢭⣻⠁⠀⠀⠀⠀⠀⠀⣼⠀⢸⠀⠀⠀⠀⢻⡋⠀⣠⣤⠴⡶⠒⠻⢦⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⢰⠋⢸⣿⣁⠀⠀⠀⠳⣤⣀⣀⣠⡴⠋⠀⠀⢸⠀⠀⠀⢀⣠⣿⣀⡠⠴⢎⠀⠙⠢⣼⡁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⢸⡀⠘⣇⣘⣿⠋⠋⠛⣳⠦⠤⣉⣛⠶⢶⣶⣛⣤⣶⣾⡿⣿⣞⠃⠀⠀⠀⠙⠒⠤⣀⣉⣙⣶⣦⣤⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠳⢤⣤⣋⡁⠀⠀⣰⠃⠀⠀⠀⣀⣼⣿⣿⡿⠛⠋⠉⠙⠻⣿⣿⣶⣤⣄⠀⠀⠀⠀⣹⣿⣿⣿⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠰⣿⣿⣿⣿⣿⣶⣧⣄⣀⣤⣾⣿⣿⣿⠟⠁⣀⣀⣠⣤⣶⣾⣿⣿⣿⣿⣿⣷⣶⣾⣿⣿⡿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠙⠛⠛⠻⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⢿⣻⣟⠛⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠹⣿⣿⣿⣿⣭⣭⣽⠛⠛⠛⠛⠛⠛⠉⠙⠯⣭⣶⣾⣿⣿⣿⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹⣿⣻⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠙⢿⣿⣿⣿⣿⣷⣤⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⠿⠿⠿⠛⠛⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠛⠛⠛⠛⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
////////////////////////////////////////////////////////////////////////
fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn body() -> HtmlElement {
    document().body().expect("document should have a body")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
/////////////////////////////////////////////////////////////////////
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⣀⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⣿⣿⣿⣿⣿⣷⣶⣤⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣀⣀⣀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣾⣿⣿⣿⢛⣛⡛⠿⣿⣿⣿⣿⣷⣤⣠⣤⣀⣀⠀⣠⣤⣴⣾⣿⣿⣿⣿⣿⣿⣿⣷⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⡇⣿⡿⠿⠿⣶⣭⣛⠿⣿⣿⠗⠀⠈⠉⠙⠻⢿⣿⣿⣿⣿⣿⣿⣿⣿⡿⢿⣷⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣿⣿⣿⣧⢻⡇⠀⠀⠈⢙⣿⠟⠋⠀⠀⠀⠀⠀⠀⠀⠀⠈⠙⠿⣿⣿⡿⢛⠱⡞⢸⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⠏⢻⣿⣿⣿⡌⣧⠀⢀⡶⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⣦⡁⢸⢣⣿⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⠃⠀⢈⣿⣿⣿⣿⣜⣷⠟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⣇⣾⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡆⠀⢠⣿⣿⣿⣿⡿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠳⣄⢸⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⠃⠀⣾⣿⣿⣿⡿⠁⠀⠀⢀⠏⠀⠀⢀⣤⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⠇⢻⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⠇⠀⢰⣿⣿⣿⣿⠃⠀⠀⠀⡾⠀⠀⠀⣼⠉⢷⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡄⢷⠀⠀⠀⠀⠀⠀⠀⠀⠀Now⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀⣰⣿⣿⣿⣿⡏⠀⠀⠀⢸⠃⠀⠀⣸⠃⠀⠈⢧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢷⠈⣇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀on⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣽⣾⣯⣍⠻⣿⡟⠀⠀⠀⠀⣿⠀⣠⠞⠁⠀⠀⠀⠈⠳⣄⠀⠀⠀⠀⠀⠀⢰⡆⠀⠀⠀⠀⢸⡇⢹⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀to
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⢸⣿⣿⣼⠁⠀⠀⠀⠀⣿⣠⠃⠀⠀⠀⢠⡀⠀⠀⠘⢦⠀⢀⣀⠀⠠⠞⠀⣶⣄⣤⡔⠈⠀⢸⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ my
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡴⠻⣿⣼⣿⣿⣧⠀⠀⠀⠀⠀⢹⡄⠀⠀⠀⠀⠀⠙⠦⣄⠀⠀⠉⠛⠉⠓⠒⠛⠛⠉⣀⣀⢠⡀⠀⢸⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ code!
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⣅⠀⠘⢷⣝⣛⣻⡄⠀⠀⠀⠀⠀⢳⠛⠿⢷⣶⣤⣤⡀⠀⠀⠀⠀⠀⠀⠀⣠⣴⣾⡿⠿⢯⡄⠉⠳⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠳⠄⣘⣻⡿⠿⣿⣶⣶⣤⣤⣴⣯⡀⠀⠀⠀⠀⠙⠃⢠⣀⣠⣀⠀⢀⡀⠉⠉⠀⠀⠀⠀⠀⠀⠀⣹⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠙⠓⠟⣩⣿⣿⣿⢿⣿⣧⣄⣀⣀⠀⢀⡴⠛⠉⠉⠙⠻⣏⠀⠀⠀⠀⠀⠀⠀⣀⡤⠾⢧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡼⠋⠈⡿⠁⠀⠈⠻⣄⡴⣋⣻⡟⠀⠀⠀⠀⠀⠀⠈⡷⣶⠶⢶⡾⣿⣉⡇⠀⠀⠘⣇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢳⣦⣄⣿⠀⠀⠀⢠⡟⠋⠉⣼⢻⡄⠀⠀⠀⠀⢀⡼⠟⠉⠀⠈⡇⠀⠈⠱⣄⣀⣀⣘⣦⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣤⣶⡶⠞⠋⠀⠰⠿⢤⡶⠖⣿⠀⠀⠀⢻⡀⠙⠓⠶⠶⣶⣿⡧⠴⠶⣾⠛⠁⠀⠀⠀⢨⣿⡉⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣷⠀⠀⠀⠀⠀⣠⠊⠙⢷⠀⠀⠀⠀⠘⢷⣄⣀⣠⣿⣿⡄⢀⣼⢻⠀⠀⠀⠀⢰⠃⡟⠈⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⣿⣿⣿⣧⡀⢀⣤⠊⠁⠀⠀⠘⡆⠀⠀⠀⠀⣀⣩⣽⡿⢹⣿⡟⠁⠘⡌⢧⠀⠀⣀⣇⣴⠇⢀⡿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⣿⣿⣿⣿⣿⣅⠀⠀⠀⠀⣠⣿⣶⡶⠚⠋⠉⠈⠉⠀⢸⣿⣷⠀⠀⠹⡌⣻⡛⠻⣅⣠⡤⠞⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠻⣿⣿⣿⣿⣿⣶⣤⣤⣿⣿⣿⣷⣶⣤⣤⣤⣤⣤⣾⣿⣿⣤⡀⢀⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠛⢻⣿⣿⣿⣿⡿⠿⢿⣿⣿⣿⣿⣿⣿⣿⠿⠿⣻⣿⣿⣿⣿⠛⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣷⣤⣴⠏⠀⠀⠀⠀⠀⠀⠀⢳⣿⣿⣿⣿⣿⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣴⣿⣿⣿⣿⣿⠟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠘⣿⣿⣿⣿⣿⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠻⠿⠿⠿⠟⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠛⠛⠛⠛⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
////////////////////////////////////////////////////////////////////////

pub struct BackendWasm {
    canvas: Rc<HtmlCanvasElement>,
}

impl BackendWasm {
    pub fn new(game_options: &GameOptions) -> Self {
        let document = document();
        let canvas = document
            .create_element("canvas")
            .expect("could not create canvas")
            .dyn_into::<HtmlCanvasElement>()
            .expect("could not dyn_into HtmlCanvasElement");
        body()
            .append_child(&canvas)
            .expect("could not add canvas to body");
        canvas.set_width(game_options.window_width);
        canvas.set_height(game_options.window_height);
        canvas
            .style()
            .set_property("border", "solid")
            .expect("cant style canvas");
        let canvas = Rc::new(canvas);
        BackendWasm { canvas }
    }
}

impl Backend for BackendWasm {
    fn create_clock(&self) -> Box<dyn Clock> {
        Box::new(WasmClock::new())
    }
    fn create_event_loop(&self, _game_options: &GameOptions) -> Box<dyn BackendEventLoop> {
        let e = EventLoopWasm {
            canvas: self.canvas.clone(),
        };
        Box::new(e)
    }
}

pub struct EventLoopWasm {
    canvas: Rc<HtmlCanvasElement>,
}

impl BackendEventLoop for EventLoopWasm {
    fn run(&mut self, mut game: Game, mut game_context: GameContext) {
        web_sys::console::log_1(&"starting game loop".into());

        let self_referencing_function: Rc<RefCell<Option<Closure<dyn FnMut()>>>> =
            Rc::new(RefCell::new(None));
        let srf_handle = self_referencing_function.clone();
        let closure =
            Closure::wrap(Box::new(move || {
                web_sys::console::log_1(&"frame!".into());
                let scene = game.scene.as_mut();
                if let Some(scene) = scene {
                    scene.init(&mut game_context);
                }

                // initialize the audio pool if the scene has queued things up
                let audio = game_context.audio.as_mut();
                if let Some(audio) = audio {
                    let _ = audio.prepare();
                }

                // TODO: this is where we need to do the closure callback dance.
                game.update(&mut game_context);
                if let Some(mut next_scene) = game_context.next_scene.take() {
                    next_scene.init(&mut game_context);
                    game.scene = Some(next_scene);
                    game.reset_for_next_scene();
                    let audio = game_context.audio.as_mut();
                    if let Some(audio) = audio {
                        audio.prepare();
                    }
                }
                game.draw(&mut game_context);
                if game_context.shutdown_flag {
                    return;
                }

                request_animation_frame(srf_handle.borrow().as_ref().expect(
                    "closure dropped before expected self referenced callback expected it",
                ));
            }) as Box<dyn FnMut()>);
        *self_referencing_function.borrow_mut() = Some(closure);
        request_animation_frame(
            self_referencing_function
                .borrow()
                .as_ref()
                .expect("code drift! closure just made is suddenly gone!"),
        );

        web_sys::console::log_1(&"goodbye wasm".into());
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
