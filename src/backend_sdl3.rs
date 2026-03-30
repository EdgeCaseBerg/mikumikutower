use crate::backend::*;
use crate::game::Game;
use crate::game::GameContext;
use crate::game_options::GameOptions;
use crate::renderer::{Color, Rect, RenderCommand, Renderer};

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use sdl3::EventPump;
use sdl3::Sdl;
use sdl3::VideoSubsystem;
use sdl3::event::Event;
use sdl3::filesystem::get_current_directory;
use sdl3::image::LoadTexture;
use sdl3::keyboard::Keycode;
use sdl3::render::Texture;
use sdl3::render::TextureCreator;
use sdl3::render::WindowCanvas;
use sdl3::video::WindowContext;

pub struct BackendSDL3 {
    sdl: Sdl,
}

type SDL3Texture = Texture<'static>;

pub struct SDL3Context {
    video: VideoSubsystem,
    window_canvas: WindowCanvas,
    textures: SDL3Textures,
}

pub struct SDL3Textures {
    texture_creator: TextureCreator<WindowContext>,
    texture_by_id: HashMap<usize, SDL3Texture>,
}

impl SDL3Textures {
    fn from(texture_creator: TextureCreator<WindowContext>) -> Self {
        SDL3Textures {
            texture_creator,
            texture_by_id: HashMap::new(),
        }
    }

    fn get_texture(&self, texture_id: usize) -> Option<&SDL3Texture> {
        self.texture_by_id.get(&texture_id)
    }

    fn load(&mut self, id: usize, path: PathBuf) {
        let tex = self.texture_creator.load_texture(path).unwrap();
        let tex = unsafe { make_static(tex) };
        self.texture_by_id.insert(id, tex);
    }
}

// Alchemy! we do this to shunt off the lifetime the sdl3 lib sets on the textures.
// both it and we know that its lifetime is tied to the texture_creator, but they
// didn't represent this by defining a lifetime on the creator, and we don't need to
// care or worry about this because the texture_creator is owned by SDL3Textures and
// so when it goes out of scope it can drop everything. I imagine I might need to
// implement a Drop for SDL3Textures to make sure that happens, but then again, its
// dropping point is _probably_ going to be the end of the program so...
unsafe fn make_static(tex: Texture) -> Texture<'static> {
    unsafe { std::mem::transmute(tex) }
}

impl BackendSDL3 {
    pub fn new(game_options: &GameOptions) -> Self {
        let sdl_handle = sdl3::init().expect("failed to init SDL");
        BackendSDL3 { sdl: sdl_handle }
    }
}

impl Backend for BackendSDL3 {
    fn create_event_loop(&self, game_options: &GameOptions) -> Box<dyn BackendEventLoop> {
        let mut event_pump = self.sdl.event_pump().unwrap();

        let video_subsystem = self.sdl.video().expect("failed to get video context");

        // Side note, window to borderless and all that would need to re-create window and derived canvases
        let window = video_subsystem
            .window(
                &game_options.name,
                game_options.window_width,
                game_options.window_height,
            )
            .position_centered()
            .build()
            .expect("failed to build window");

        let canvas = window.into_canvas();
        let mut textures = SDL3Textures::from(canvas.texture_creator());

        // TODO: should probably move this out somewhere else
        let base = get_current_directory().expect("cant get base path");
        let base = base.join(game_options.assets_path.clone());
        let chaim_dir = base.join("chaim-vester");
        let portraits = chaim_dir.join("portraits-spritesheet.png");
        let miku = base.join("dance.png");

        // TODO: move constants out somewhere re-useable and referenceable
        textures.load(0, miku);
        textures.load(1, portraits);

        let e = EventLoopSDL3 {
            event_pump,
            context: Rc::new(RefCell::new(SDL3Context {
                video: video_subsystem,
                window_canvas: canvas,
                textures,
            })),
        };
        Box::new(e)
    }
}

pub struct EventLoopSDL3 {
    event_pump: EventPump,
    context: Rc<RefCell<SDL3Context>>, // in a rc + refcell because we need to be able to pass around &mut for shared stuff.
}

impl BackendEventLoop for EventLoopSDL3 {
    fn run(&mut self, game: &mut Game, game_context: &mut GameContext) {
        'running: loop {
            // TODO: merge events into state tracking system that doesn't exist yet
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
            game.update(game_context);

            game.draw(game_context);
        }
    }

    fn new_renderer(&self, game_options: &GameOptions) -> Box<dyn Renderer> {
        let r = RendererSDL3 {
            context: self.context.clone(),
        };
        Box::new(r)
    }
}

struct RendererSDL3 {
    context: Rc<RefCell<SDL3Context>>,
}

impl Renderer for RendererSDL3 {
    fn name(&self) -> String {
        "SDL3 Renderer".to_string()
    }

    fn clear(&mut self, color: Color) {
        let mut ctx = self.context.borrow_mut();
        ctx.window_canvas.set_draw_color(color.to_sdl3());
        ctx.window_canvas.clear();
    }

    fn present(&mut self) {
        let mut ctx = self.context.borrow_mut();
        ctx.window_canvas.present();
    }
}

impl Color {
    pub fn to_sdl3(&self) -> sdl3::pixels::Color {
        let (r, g, b, a) = (*self).into();
        sdl3::pixels::Color::RGBA(r, g, b, a)
    }
}