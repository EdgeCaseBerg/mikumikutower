use crate::game::Game;
use crate::game_options::GameOptions;

use std::cell::RefCell;
use std::rc::Rc;

// Colors are defined in sRGB within 0.0 - 1.0
// While SDL provides one of these, for future proofing against playing with other renderers, define our own.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const fn Black() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }
}

// Generic trait that we'll make an SDL version of for fun.
// and in order to be able to write code in a generic way while
// swapping out graphics library for fun as needed.
pub trait Renderer {
    fn name(&self) -> String;
    fn clear(&mut self, color: Color);
    fn present(&mut self);
}

// TODO: move to own file once done.

use sdl3::EventPump;
use sdl3::Sdl;
use sdl3::VideoSubsystem;
use sdl3::render::WindowCanvas;
use sdl3::video::Window;

pub struct BackendSDL3 {
    sdl: Sdl,
}

pub struct SDL3Context {
    video: VideoSubsystem,
    window_canvas: WindowCanvas,
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

        let e = EventLoopSDL3 {
            event_pump,
            context: Rc::new(RefCell::new(SDL3Context {
                video: video_subsystem,
                window_canvas: window.into_canvas(),
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
    fn run(&mut self, game: &mut Game) {}
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
        // color conversion
        let mut ctx = self.context.borrow_mut();
        ctx.window_canvas
            .set_draw_color(sdl3::pixels::Color::RGB(0, 255, 255));
        ctx.window_canvas.clear();
    }

    fn present(&mut self) {
        let mut ctx = self.context.borrow_mut();
        ctx.window_canvas.present();
    }
}
