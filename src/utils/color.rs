use macroquad::color::Color;
use serde::{Deserialize, Serialize};

#[macro_export]
macro_rules! gcolor_u8 {
    ($r:expr, $g:expr, $b:expr, $a:expr) => {
        GColor {
            r: $r as f32 / 255.,
            g: $g as f32 / 255.,
            b: $b as f32 / 255.,
            a: $a as f32 / 255.,
        }
    };
}

#[derive(Clone, Serialize, Deserialize, Copy, PartialEq)]
pub struct GColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl GColor {
    pub fn blend(&self, other: &Self, factor: f32) -> Self {
        assert!((0. ..=1.).contains(&factor));

        Self::from_rgba(
            (self.r_u8() as f32 * (1.0 - factor) + other.r_u8() as f32 * factor) as u8,
            (self.g_u8() as f32 * (1.0 - factor) + other.g_u8() as f32 * factor) as u8,
            (self.b_u8() as f32 * (1.0 - factor) + other.b_u8() as f32 * factor) as u8,
            (self.a_u8() as f32 * (1.0 - factor) + other.a_u8() as f32 * factor) as u8,
        )
    }

    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.,
            g: g as f32 / 255.,
            b: b as f32 / 255.,
            a: a as f32 / 255.,
        }
    }

    pub fn r_u8(&self) -> u8 {
        (self.r * 255.) as u8
    }

    pub fn g_u8(&self) -> u8 {
        (self.g * 255.) as u8
    }

    pub fn b_u8(&self) -> u8 {
        (self.b * 255.) as u8
    }

    pub fn a_u8(&self) -> u8 {
        (self.a * 255.) as u8
    }

    pub fn to_mq(&self) -> Color {
        Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }

    pub fn to_rgba_arr(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn from_rgba_arr([r, g, b, a]: [f32; 4]) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_hex(hex: u32) -> Self {
        let r = ((hex >> 16) & 0xFF) as f32 / 255.;
        let g = ((hex >> 8) & 0xFF) as f32 / 255.;
        let b = (hex & 0xFF) as f32 / 255.;

        Self { r, g, b, a: 1. }
    }
}
