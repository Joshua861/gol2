use macroquad::rand::rand;

pub use color::*;
pub use tiny_str::*;

mod color;
mod tiny_str;

pub fn rand_bool() -> bool {
    rand() % 2 == 0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2I {
    pub x: isize,
    pub y: isize,
}
