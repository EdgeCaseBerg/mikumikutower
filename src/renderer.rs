
use std::ops::{Add, Sub};

// Colors are defined in sRGB within 0.0 - 1.0
// While SDL provides one of these, for future proofing against playing with other renderers, define our own.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl From<Color> for (u8, u8, u8, u8) {
    fn from(color: Color) -> (u8, u8, u8, u8) {
        let r = (color.r * 255.0) as u8;
        let g = (color.g * 255.0) as u8;
        let b = (color.b * 255.0) as u8;
        let a = (color.a * 255.0) as u8;
        (r, g, b, a)
    }
}

impl Color {
    pub fn to_sdl3(&self) -> sdl3::pixels::Color {
        let (r, g, b, a) = (*self).into();
        sdl3::pixels::Color::RGBA(r, g, b, a)
    }
}

impl Color {
    pub const fn black() -> Color {
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

pub enum RenderCommand {
    DrawRect {
        texture_id: usize,
        source: Rect,
        destination: Rect,
    },
}
