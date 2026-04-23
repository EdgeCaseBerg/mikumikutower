pub mod backend;
pub mod backend_sdl3;
pub mod constants;
pub mod game;
pub mod game_options;
pub mod grid_layout;
pub mod renderer;
pub mod scene;

use std::ops::{Add, Sub};

use crate::backend::init_backend;
use crate::game::Game;
use crate::game_options::GameOptions;
use crate::scene::Scene;
use crate::scene::level::LevelScene;
use crate::scene::loading::TestScene;

extern crate sdl3;

#[derive(Debug, Clone)]
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

    fn get_rect(&self) -> Rect {
        let [x, y, w, h] = self.get_rect_for();
        Rect::new(x as isize, y as isize, w as isize, h as isize)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rect<T: PartialOrd + Copy + Add<Output = T> + Sub<Output = T> = isize> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl<T: PartialOrd + Copy + Add<Output = T> + Sub<Output = T>> Rect<T> {
    // (x, y) are the corner from which width and height expand from (x + width = x2, similar for y)
    // so its the bottom left corner in a positive coordinate space and the top left in a quadrant 4 space (like most images)
    #[inline(always)]
    pub fn new(x: T, y: T, width: T, height: T) -> Rect<T> {
        Rect {
            x,
            y,
            width,
            height,
        }
    }
}

pub fn hello_sdl(game_options: &GameOptions, game: &mut Game) {
    let backend = init_backend(&game_options);
    let mut event_loop = backend.create_event_loop(&game_options);
    let mut game_context = crate::game::GameContext::default();
    game_context.screen_size = (game_options.window_width, game_options.window_height);
    let renderer = event_loop.new_renderer(game_options);
    game_context.renderer = Some(renderer);
    game.scene = Some(Box::new(TestScene::default()));
    game.scene = Some(Box::new(LevelScene::default()));
    event_loop.run(game, &mut game_context);
}
