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
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use std::time::Duration; // this is probably going to bite us later.

use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{
    CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlDivElement, HtmlElement,
    HtmlImageElement, Window,
};

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

struct WasmContext {
    storage: Rc<HtmlDivElement>,
    context: Rc<CanvasRenderingContext2d>,
    canvas: Rc<HtmlCanvasElement>,
    document: Rc<Document>,
    texture_id_to_image: HashMap<TextureId, HtmlImageElement>,
}

impl WasmContext {
    fn get_image(&self, id: TextureId) -> Option<&HtmlImageElement> {
        let Some(html_image_element) = self.texture_id_to_image.get(&id) else {
            return None;
        };
        if !html_image_element.complete() {
            return None;
        }
        Some(html_image_element)
    }
}

struct AssetLoaderWasm {
    base_path: PathBuf,
    wasm_context: Rc<RefCell<WasmContext>>,
}

impl AssetLoaderWasm {
    fn new(game_options: &GameOptions, wasm_context: Rc<RefCell<WasmContext>>) -> Self {
        let path = game_options.assets_path.clone();

        Self {
            base_path: path,
            wasm_context: wasm_context.clone(),
        }
    }
}

fn pathbuf_to_url(p: &PathBuf) -> String {
    let mut s = String::new();
    let len = p.iter().count();
    for (index, path_part) in p.iter().enumerate() {
        s.push_str(&path_part.to_string_lossy());
        if index < len - 1 {
            s.push_str("/");
        }
    }
    s
}

impl AssetLoader for AssetLoaderWasm {
    fn ensure_texture_spritesheet_loaded(&mut self, id: TextureId) {
        let context = &mut *self.wasm_context.borrow_mut();

        if let Some(_) = context.texture_id_to_image.get(&id) {
            web_sys::console::log_1(&format!("texture id {} already loaded", id.0).into());
            return;
        }
        web_sys::console::log_1(&format!("loading texture id {}", id.0).into());
        let img = context
            .document
            .create_element("img")
            .expect("could not create img")
            .dyn_into::<HtmlImageElement>()
            .expect("could not dyn_into HtmlImageElement");
        img.set_name(&format!("texture-{}", id.0));
        let path = id_to_relative_path(id);
        let path = self.base_path.join(path);
        img.set_src(&pathbuf_to_url(&path));
        let result = context.storage.append_child(&img);
        if let Ok(_) = result {
            context.texture_id_to_image.insert(id, img);
            web_sys::console::log_1(&format!("texture loaded for texture id {}", id.0).into());
        } else if let Err(e) = result {
            web_sys::console::log_1(
                &format!("error loading texture id {} {:?}", id.0, e.as_string()).into(),
            );
        }
    }
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
    fn sleep(&self) {
        // do nothing. the request animation frame loop sleeps for us.
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
        canvas
            .style()
            .set_property("background", "black")
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
        let document = document();
        let div = document
            .create_element("div")
            .expect("could not create div")
            .dyn_into::<HtmlDivElement>()
            .expect("could not dyn_into HtmlDivElement");
        div.style()
            .set_property("display", "none")
            .expect("couldnt hide div to load images into");
        body()
            .append_child(&div)
            .expect("could not add canvas to body");
        let context = (*self.canvas)
            .get_context("2d")
            .expect("could not find 2d context")
            .expect("could not get context, was none")
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .expect("could not dyn_into element into CanvasRenderingContext2d");

        let wasm_context = Rc::new(RefCell::new(WasmContext {
            context: context.into(),
            canvas: self.canvas.clone(),
            document: document.into(),
            storage: div.into(),
            texture_id_to_image: HashMap::new(),
        }));
        let e = EventLoopWasm {
            canvas: self.canvas.clone(),
            wasm_context: wasm_context,
        };
        Box::new(e)
    }
}

pub struct EventLoopWasm {
    canvas: Rc<HtmlCanvasElement>,
    wasm_context: Rc<RefCell<WasmContext>>,
}

impl BackendEventLoop for EventLoopWasm {
    fn run(&mut self, mut game: Game, mut game_context: GameContext) {
        web_sys::console::log_1(&"starting game loop".into());

        // First time load setup
        let scene = game.scene.as_mut();
        if let Some(scene) = scene {
            scene.init(&mut game_context);
        }

        // setup input processing
        // initialize the audio pool if the scene has queued things up
        let audio = game_context.audio.as_mut();
        if let Some(audio) = audio {
            let _ = audio.prepare();
        }

        let left_pressed = Rc::new(RefCell::new(false));
        let right_pressed = Rc::new(RefCell::new(false));
        let mouse_coordinates = Rc::new(RefCell::new(None));
        {
            let left_pressed = left_pressed.clone();
            let right_pressed = right_pressed.clone();
            let mouse_coordinates = mouse_coordinates.clone();
            let canvas = self.canvas.clone();
            let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
                match event.button() {
                    0 => {
                        *left_pressed.borrow_mut() = true;
                    }
                    2 => *right_pressed.borrow_mut() = true,
                    _ => {}
                }
                let rect = canvas.get_bounding_client_rect();
                let x = (event.client_x() as f64 - rect.left())
                    * (canvas.width() as f64 / rect.width());

                let y = (event.client_y() as f64 - rect.top())
                    * (canvas.height() as f64 / rect.height());
                *mouse_coordinates.borrow_mut() = Some((x as f32, y as f32));
            });
            self.canvas
                .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }
        {
            let mouse_coordinates = mouse_coordinates.clone();
            let canvas = self.canvas.clone();
            let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
                let rect = canvas.get_bounding_client_rect();
                let x = (event.client_x() as f64 - rect.left())
                    * (canvas.width() as f64 / rect.width());

                let y = (event.client_y() as f64 - rect.top())
                    * (canvas.height() as f64 / rect.height());
                *mouse_coordinates.borrow_mut() = Some((x as f32, y as f32));
            });
            self.canvas
                .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }
        {
            let left_pressed = left_pressed.clone();
            let right_pressed = right_pressed.clone();
            let mouse_coordinates = mouse_coordinates.clone();
            let canvas = self.canvas.clone();
            let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
                // event.offset_x() as f64, event.offset_y() as f64
                match event.button() {
                    0 => {
                        *left_pressed.borrow_mut() = false;
                    }
                    2 => {
                        *right_pressed.borrow_mut() = false;
                    }
                    _ => {}
                }
                let rect = canvas.get_bounding_client_rect();
                let x = (event.client_x() as f64 - rect.left())
                    * (canvas.width() as f64 / rect.width());

                let y = (event.client_y() as f64 - rect.top())
                    * (canvas.height() as f64 / rect.height());
                *mouse_coordinates.borrow_mut() = Some((x as f32, y as f32));
            });
            self.canvas
                .add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }

        let self_referencing_function: Rc<RefCell<Option<Closure<dyn FnMut()>>>> =
            Rc::new(RefCell::new(None));
        let srf_handle = self_referencing_function.clone();
        let closure =
            Closure::wrap(Box::new(move || {
                // Any events?
                game_context.mouse_context.update(
                    *left_pressed.borrow(),
                    *right_pressed.borrow(),
                    *mouse_coordinates.borrow(),
                );

                // Run updates.
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

        // LOOOOOOOOOP
        request_animation_frame(
            self_referencing_function
                .borrow()
                .as_ref()
                .expect("code drift! closure just made is suddenly gone!"),
        );

        web_sys::console::log_1(&"goodbye wasm".into());
    }

    fn new_renderer(&self, _game_options: &GameOptions) -> Box<dyn Renderer> {
        let r = RendererWasm {
            commands: vec![],
            wasm_context: self.wasm_context.clone(),
        };
        Box::new(r)
    }

    fn create_asset_loader(&self, game_options: &GameOptions) -> Box<dyn AssetLoader> {
        let a = AssetLoaderWasm::new(game_options, self.wasm_context.clone());
        Box::new(a)
    }

    fn create_audio(&self, game_options: &GameOptions) -> Box<dyn Audio> {
        let s = WasmSounds::new(game_options);
        Box::new(s)
    }
}

struct RendererWasm {
    wasm_context: Rc<RefCell<WasmContext>>,
    commands: Vec<RenderCommand>,
}

impl RendererWasm {
    fn process_commands(&mut self) {
        for cmd in self.commands.drain(..) {
            match cmd {
                RenderCommand::DrawRect {
                    texture_id,
                    source,
                    destination,
                } => {
                    let ctx = &mut *self.wasm_context.borrow_mut();
                    if let Some(html_image_element) = ctx.get_image(texture_id) {
                        let (sx, sy, sw, sh) = (source.x, source.y, source.width, source.height);
                        let (dx, dy, dw, dh) = (
                            destination.x,
                            destination.y,
                            destination.width,
                            destination.height,
                        );
                        let context2d = ctx.context.clone();
                        let result = context2d.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                            html_image_element,
                            sx as f64,
                            sy as f64,
                            sw as f64,
                            sh as f64,
                            dx as f64,
                            dy as f64,
                            dw as f64,
                            dh as f64,
                        );
                        if let Err(e) = result {
                            let msg = format!(
                                "failed to draw texture {} {:?}",
                                texture_id.0,
                                e.as_string()
                            );
                            web_sys::console::log_1(&msg.into());
                        }
                    }
                }
            }
        }
    }
}

impl Renderer for RendererWasm {
    fn name(&self) -> String {
        "WASM Renderer".to_string()
    }

    fn clear(&mut self, _color: Color) {
        let ctx = &mut *self.wasm_context.borrow_mut();
        let context2d = ctx.context.clone();
        context2d.clear_rect(
            0.,
            0.,
            (*ctx.canvas).width().into(),
            (*ctx.canvas).height().into(),
        );
    }

    fn present(&mut self) {
        self.process_commands();
    }

    fn send_command(&mut self, cmd: RenderCommand) {
        self.commands.push(cmd);
    }
}
