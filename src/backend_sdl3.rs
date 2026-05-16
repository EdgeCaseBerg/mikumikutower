use crate::Rect;
use crate::asset_loader::AssetLoader;
use crate::backend::*;
use crate::constants::*;
use crate::game::Game;
use crate::game::GameContext;
use crate::game_options::GameOptions;
use crate::renderer::{Color, RenderCommand, Renderer};

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use sdl3::EventPump;
use sdl3::Sdl;
use sdl3::VideoSubsystem;
use sdl3::event::{Event, WindowEvent};
use sdl3::filesystem::get_current_directory;
use sdl3::image::LoadTexture;
use sdl3::keyboard::Keycode;
use sdl3::mouse::MouseButton;
use sdl3::render::Texture;
use sdl3::render::TextureCreator;
use sdl3::render::WindowCanvas;
use sdl3::video::WindowContext;

pub struct BackendSDL3 {
    sdl: Sdl,
}

type SDL3Texture = Texture<'static>;

pub struct SDL3Context {
    // Note: textures MUST be declared ABOVE window_canvas because
    // drop order is top to bottom and all textures need to be dropped
    // BEFORE the canvas is dropped
    textures: SDL3Textures,
    window_canvas: WindowCanvas,
    video: VideoSubsystem,
}

pub struct SDL3Textures {
    texture_by_id: HashMap<usize, SDL3Texture>,
    // ORDER OF STRUCT IS IMPORTANT BECAUSE OF DROP ORDER
    // WE DROP THE TEXTURES PRIOR TO THE CREATOR GOING AWAY
    texture_creator: TextureCreator<WindowContext>,
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
        let tex = make_static(tex);
        self.texture_by_id.insert(id, tex);
    }
}

struct AssetLoaderSDL3 {
    context: Rc<RefCell<SDL3Context>>,
    base_path: PathBuf,
}

impl AssetLoaderSDL3 {
    fn new(context: Rc<RefCell<SDL3Context>>, game_options: &GameOptions) -> Self {
        let base = get_current_directory().expect("cant get base path");
        let base = base.join(game_options.assets_path.clone());
        Self {
            context,
            base_path: base,
        }
    }
}

impl AssetLoader for AssetLoaderSDL3 {
    fn ensure_texture_spritesheet_loaded(&mut self, id: usize) {
        let ctx = &mut *self.context.borrow_mut();
        if !ctx.textures.get_texture(id).is_none() {
            return;
        }
        let asset_path = id_to_relative_path(id);
        let asset_path = self.base_path.join(asset_path);
        ctx.textures.load(id, asset_path);
    }
}

// Alchemy! we do this to shunt off the lifetime the sdl3 lib sets on the textures.
// both it and we know that its lifetime is tied to the texture_creator, but they
// didn't represent this by defining a lifetime on the creator, and we don't need to
// care or worry about this because the texture_creator is owned by SDL3Textures and
// so when it goes out of scope it can drop everything. I imagine I might need to
// implement a Drop for SDL3Textures to make sure that happens, but then again, its
// dropping point is _probably_ going to be the end of the program so... eh.
// The SDL3 docs says we should destroy it when we're done https://wiki.libsdl.org/SDL3_image/IMG_LoadTexture
//
fn make_static(tex: Texture) -> Texture<'static> {
    unsafe { std::mem::transmute(tex) }
}

impl BackendSDL3 {
    pub fn new(_game_options: &GameOptions) -> Self {
        let sdl_handle = sdl3::init().expect("failed to init SDL");
        BackendSDL3 { sdl: sdl_handle }
    }
}

impl Backend for BackendSDL3 {
    fn create_event_loop(&self, game_options: &GameOptions) -> Box<dyn BackendEventLoop> {
        let event_pump = self.sdl.event_pump().unwrap();

        let video_subsystem = self.sdl.video().expect("failed to get video context");

        // Side note, window to borderless and all that would need to re-create window and derived canvases
        let window = video_subsystem
            .window(
                &game_options.name,
                game_options.window_width,
                game_options.window_height,
            )
            .position_centered()
            .resizable()
            .build()
            .expect("failed to build window");

        let canvas = window.into_canvas();
        let mut textures = SDL3Textures::from(canvas.texture_creator());

        // If we end up having some custom form of cursor for each scene then we can do this
        // self.sdl.mouse().show_cursor(false);

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
        let scene = game.scene.as_mut();
        if let Some(scene) = scene {
            scene.init(game_context);
        }

        'running: loop {
            // TODO: merge events into state tracking system that doesn't exist yet
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::MouseMotion {
                        mousestate,
                        x,
                        y,
                        xrel,
                        yrel,
                        ..
                    } => {
                        game_context.mouse_context.update(
                            mousestate.left(),
                            mousestate.right(),
                            Some((x, y)),
                        );
                    }
                    Event::MouseButtonDown {
                        mouse_btn, x, y, ..
                    } => {
                        game_context.mouse_context.update(
                            mouse_btn == MouseButton::Left,
                            mouse_btn == MouseButton::Right,
                            Some((x, y)),
                        );
                    }
                    Event::Window { win_event, .. } => match win_event {
                        WindowEvent::Resized(w, h) => {
                            game_context.screen_size = (w as u32, h as u32);
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
            game.update(game_context);
            if let Some(mut next_scene) = game_context.next_scene.take() {
                next_scene.init(game_context);
                game.scene = Some(next_scene);
                game.reset_for_next_scene();
            }
            game.draw(game_context);
        }
    }

    fn new_renderer(&self, _game_options: &GameOptions) -> Box<dyn Renderer> {
        let r = RendererSDL3 {
            context: self.context.clone(),
            commands: Vec::with_capacity(32),
        };
        Box::new(r)
    }

    fn create_asset_loader(&self, game_options: &GameOptions) -> Box<dyn AssetLoader> {
        let a = AssetLoaderSDL3::new(self.context.clone(), game_options);
        Box::new(a)
    }
}

struct RendererSDL3 {
    context: Rc<RefCell<SDL3Context>>,
    commands: Vec<RenderCommand>,
}

impl RendererSDL3 {
    // Internally used before presenting. Drains all commands
    // in order to enque all the work to SDL3 that we want done
    // per frame.
    fn process_commands(&mut self) {
        for cmd in self.commands.drain(..) {
            match cmd {
                RenderCommand::DrawRect {
                    texture_id,
                    source,
                    destination,
                } => {
                    let ctx = &mut *self.context.borrow_mut();
                    if let Some(texture) = ctx.textures.get_texture(texture_id) {
                        let src: sdl3::rect::Rect = source.into();
                        let dst: sdl3::rect::Rect = destination.into();
                        ctx.window_canvas
                            .copy(texture, src, dst)
                            .unwrap_or_else(|_| {
                                let _ = &format!("failed to draw texture {}", texture_id);
                            });
                    }
                }
            }
        }
    }
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
        self.process_commands();
        let mut ctx = self.context.borrow_mut();
        ctx.window_canvas.present();
    }

    fn send_command(&mut self, cmd: RenderCommand) {
        self.commands.push(cmd);
    }
}

impl Color {
    pub fn to_sdl3(&self) -> sdl3::pixels::Color {
        let (r, g, b, a) = (*self).into();
        sdl3::pixels::Color::RGBA(r, g, b, a)
    }
}

impl From<Rect> for sdl3::rect::Rect {
    fn from(r: Rect) -> Self {
        sdl3::rect::Rect::new(r.x as i32, r.y as i32, r.width as u32, r.height as u32)
    }
}
