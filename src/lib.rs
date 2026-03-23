pub mod game_options;
use std::time::Duration;

use crate::game_options::GameOptions;

pub fn hello() {
    println!("Hi");
}

extern crate sdl3;

use sdl3::event::Event;
use sdl3::filesystem::get_current_directory;
use sdl3::image::LoadTexture;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::pixels::PixelFormat;
use sdl3::rect::Rect;
use sdl3::render::{Canvas, Texture};
use sdl3::surface::Surface;
use sdl3::video::Window;

#[derive(Debug)]
struct SpriteInfo {
    start_x: u32,
    start_y: u32,
    width: u32,
    height: u32,
    frames: u32,
    current_frame: u32,
    framerate_per_second: u32,
    delta: u32,
}

impl SpriteInfo {
    fn advance(&mut self, delta: u32) {
        self.delta = self.delta.wrapping_add(delta);
        if self.delta > self.framerate_per_second {
            self.current_frame += 1;
            self.delta = 0;
        }
        // Always loop for now.
        if self.current_frame >= self.frames {
            self.current_frame = 0;
        }
    }

    fn get_rect_for(&self) -> [u32; 4] {
        let x_offset = self.start_x + self.width * self.current_frame;
        [x_offset, self.start_y, self.width, self.height]
    }
}

pub fn hello_sdl(game_options: &GameOptions) {
    let base = get_current_directory().expect("cant get base path");
    let chaim_dir = base.join("assets").join("chaim-vester");
    let portraits = chaim_dir.join("portraits-spritesheet.png");
    let miku = base.join("assets").join("dance.png");

    let sdl_context = sdl3::init().expect("failed to init SDL");
    let video_subsystem = sdl_context.video().expect("failed to get video context");

    let window = video_subsystem
        .window(
            "Miku Miku Tower",
            game_options.window_width,
            game_options.window_height,
        )
        .position_centered()
        .build()
        .expect("failed to build window");

    let mut canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();
    let portraits_texture = texture_creator.load_texture(portraits).unwrap();
    let miku_texture = texture_creator.load_texture(miku).unwrap();
    let mut miku_sprite = SpriteInfo {
        start_x: 0,
        start_y: 0,
        width: 71,
        height: 54,
        frames: 6,
        current_frame: 0,
        framerate_per_second: 1_000_000_000u32 / 15,
        delta: 0,
    };
    println!("{:?}", miku_sprite);

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let texture_sheet_width = portraits_texture.width() - portraits_texture.width() / 7;
    let texture_sheet_height = portraits_texture.height() - portraits_texture.height() / 2;

    let mut delta = 0;
    'running: loop {
        miku_sprite.advance(delta);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        let [x, y, w, h] = miku_sprite.get_rect_for();
        canvas.clear();
        canvas
            .copy(
                &miku_texture,
                Rect::new(x as i32, y as i32, w, h),
                Rect::new(200, 600, w, h),
            )
            .expect("failed to draw portrait texture");
        canvas
            .copy(
                &portraits_texture,
                Rect::new(0, 0, texture_sheet_width, texture_sheet_height),
                Rect::new(0, 0, texture_sheet_width, texture_sheet_height),
            )
            .expect("failed to draw portrait texture");
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        delta = delta.wrapping_add(1_000_000_000u32 / 60);
    }
}
