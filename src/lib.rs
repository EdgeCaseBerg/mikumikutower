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

pub fn hello_sdl(game_options: &GameOptions) {
    let base = get_current_directory().expect("cant get base path");
    let chaim_dir = base.join("assets").join("chaim-vester");
    let image = chaim_dir.join("portraits-spritesheet.png");

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
    let texture = texture_creator.load_texture(image).unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    let mut j = 0;
    let texture_sheet_width = texture.width() - texture.width() / 7;
    'running: loop {
        i = (i + 1) % game_options.window_width;
        j = (j + 1) % texture_sheet_width;
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
        canvas.clear();
        canvas
            .copy(
                &texture,
                Rect::new(j as i32, 0, 413, 402),
                Rect::new(i as i32, 0, 413, 402),
            )
            .expect("failed t draw texture");
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
