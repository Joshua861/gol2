use crate::{
    game::Game,
    gcolor_u8, notify_info, tiny_str,
    utils::{GColor, TinyStr},
};
use serde::{Deserialize, Serialize};

impl Game {
    pub fn apply_color_scheme(&mut self) {
        notify_info!(
            self,
            "Applying color scheme: {}",
            self.config.color_scheme.name
        );

        let color_scheme = self.config.color_scheme.clone();
        let config = &mut self.config;

        config.bg_color = color_scheme.bg_color;
        config.dead_color = color_scheme.dead_color;
        config.alive_color = color_scheme.alive_color;
        config.hot_color = color_scheme.hot_color;
        config.text_color = color_scheme.text_color;
        config.highlight_color = color_scheme.highlight_color;
        config.selection_color = color_scheme.line_color;
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorScheme {
    pub name: TinyStr,
    pub bg_color: GColor,
    pub dead_color: GColor,
    pub alive_color: GColor,
    pub hot_color: GColor,
    pub text_color: GColor,
    pub highlight_color: GColor,
    pub line_color: GColor,
}

pub const SOLARIZED: ColorScheme = ColorScheme {
    name: tiny_str!("Solarized"),
    bg_color: gcolor_u8!(0x00, 0x2B, 0x36, 255),
    dead_color: gcolor_u8!(0x07, 0x36, 0x42, 255),
    alive_color: gcolor_u8!(0xFD, 0xF6, 0xE3, 255),
    hot_color: gcolor_u8!(0x58, 0x6E, 0x75, 255),
    text_color: gcolor_u8!(0xFF, 0xFF, 0xFF, 255),
    highlight_color: gcolor_u8!(0xFF, 0xFF, 0xFF, 50),
    line_color: gcolor_u8!(0x26, 0x8B, 0xD2, 100),
};

pub const DARK: ColorScheme = ColorScheme {
    name: tiny_str!("Dark"),
    bg_color: gcolor_u8!(10, 10, 10, 255),
    dead_color: gcolor_u8!(15, 15, 15, 255),
    alive_color: gcolor_u8!(240, 240, 240, 255),
    hot_color: gcolor_u8!(50, 50, 50, 255),
    text_color: gcolor_u8!(0xFF, 0xFF, 0xFF, 255),
    highlight_color: gcolor_u8!(0xFF, 0xFF, 0xFF, 50),
    line_color: gcolor_u8!(112, 158, 238, 100),
};

pub const LIGHT: ColorScheme = ColorScheme {
    name: tiny_str!("Light"),
    bg_color: gcolor_u8!(230, 230, 230, 255),
    dead_color: gcolor_u8!(240, 240, 240, 255),
    alive_color: gcolor_u8!(10, 10, 10, 255),
    hot_color: gcolor_u8!(180, 180, 180, 255),
    text_color: gcolor_u8!(0xFF, 0xFF, 0xFF, 255),
    highlight_color: gcolor_u8!(0xFF, 0xFF, 0xFF, 50),
    line_color: gcolor_u8!(10, 38, 117, 100),
};

pub const BLUE: ColorScheme = ColorScheme {
    name: tiny_str!("Blue"),
    bg_color: gcolor_u8!(0, 0, 10, 255),
    dead_color: gcolor_u8!(0, 0, 0, 255),
    alive_color: gcolor_u8!(255, 255, 255, 255),
    hot_color: gcolor_u8!(0, 0, 255, 255),
    text_color: gcolor_u8!(0xFF, 0xFF, 0xFF, 255),
    highlight_color: gcolor_u8!(0xFF, 0xFF, 0xFF, 50),
    line_color: gcolor_u8!(215, 139, 45, 100),
};

pub const RED: ColorScheme = ColorScheme {
    name: tiny_str!("Red"),
    bg_color: gcolor_u8!(10, 0, 0, 255),
    dead_color: gcolor_u8!(0, 0, 0, 255),
    alive_color: gcolor_u8!(255, 255, 255, 255),
    hot_color: gcolor_u8!(255, 0, 0, 255),
    text_color: gcolor_u8!(0xFF, 0xFF, 0xFF, 255),
    highlight_color: gcolor_u8!(0xFF, 0xFF, 0xFF, 50),
    line_color: gcolor_u8!(90, 139, 223, 100),
};

pub const GREEN: ColorScheme = ColorScheme {
    name: tiny_str!("Green"),
    bg_color: gcolor_u8!(0, 10, 0, 255),
    dead_color: gcolor_u8!(0, 0, 0, 255),
    alive_color: gcolor_u8!(255, 255, 255, 255),
    hot_color: gcolor_u8!(0, 255, 0, 255),
    text_color: gcolor_u8!(0xFF, 0xFF, 0xFF, 255),
    highlight_color: gcolor_u8!(0xFF, 0xFF, 0xFF, 50),
    line_color: gcolor_u8!(222, 94, 71, 100),
};

pub const YELLOW: ColorScheme = ColorScheme {
    name: tiny_str!("Yellow"),
    bg_color: gcolor_u8!(10, 10, 0, 255),
    dead_color: gcolor_u8!(0, 0, 0, 255),
    alive_color: gcolor_u8!(255, 255, 255, 255),
    hot_color: gcolor_u8!(255, 255, 0, 255),
    text_color: gcolor_u8!(0xFF, 0xFF, 0xFF, 255),
    highlight_color: gcolor_u8!(0xFF, 0xFF, 0xFF, 50),
    line_color: gcolor_u8!(189, 90, 217, 100),
};

pub const PURPLE: ColorScheme = ColorScheme {
    name: tiny_str!("Purple"),
    bg_color: gcolor_u8!(10, 0, 10, 255),
    dead_color: gcolor_u8!(0, 0, 0, 255),
    alive_color: gcolor_u8!(255, 255, 255, 255),
    hot_color: gcolor_u8!(255, 0, 255, 255),
    text_color: gcolor_u8!(0xFF, 0xFF, 0xFF, 255),
    highlight_color: gcolor_u8!(0xFF, 0xFF, 0xFF, 50),
    line_color: gcolor_u8!(189, 124, 32, 100),
};

pub const GRUVBOX_YELLOW: ColorScheme = ColorScheme {
    name: tiny_str!("Gruvbox yellow"),
    bg_color: gcolor_u8!(0x1D, 0x20, 0x21, 255),
    dead_color: gcolor_u8!(0x28, 0x28, 0x28, 255),
    alive_color: gcolor_u8!(0xEB, 0xDB, 0xB2, 255),
    hot_color: gcolor_u8!(0xFA, 0xBD, 0x2F, 255),
    text_color: gcolor_u8!(0xFB, 0xF1, 0xC7, 255),
    highlight_color: gcolor_u8!(0xFB, 0xF1, 0xC7, 255),
    line_color: gcolor_u8!(0x45, 0x85, 0x88, 100),
};

pub const GRUVBOX_BLUE: ColorScheme = ColorScheme {
    name: tiny_str!("Gruvbox blue"),
    bg_color: gcolor_u8!(0x1D, 0x20, 0x21, 255),
    dead_color: gcolor_u8!(0x28, 0x28, 0x28, 255),
    alive_color: gcolor_u8!(0xEB, 0xDB, 0xB2, 255),
    hot_color: gcolor_u8!(0x45, 0x85, 0x88, 255),
    text_color: gcolor_u8!(0xFB, 0xF1, 0xC7, 255),
    highlight_color: gcolor_u8!(0xFB, 0xF1, 0xC7, 255),
    line_color: gcolor_u8!(0xD7, 0x99, 0x21, 100),
};

pub const GRUVBOX_GREY: ColorScheme = ColorScheme {
    name: tiny_str!("Gruvbox grey"),
    bg_color: gcolor_u8!(0x1D, 0x20, 0x21, 255),
    dead_color: gcolor_u8!(0x28, 0x28, 0x28, 255),
    alive_color: gcolor_u8!(0xEB, 0xDB, 0xB2, 255),
    hot_color: gcolor_u8!(0x50, 0x49, 0x45, 255),
    text_color: gcolor_u8!(0xFB, 0xF1, 0xC7, 255),
    highlight_color: gcolor_u8!(0xFB, 0xF1, 0xC7, 255),
    line_color: gcolor_u8!(0xD6, 0x5D, 0x0E, 100),
};

pub const THEMES: &[ColorScheme] = &[
    DARK,
    LIGHT,
    RED,
    GREEN,
    BLUE,
    YELLOW,
    PURPLE,
    SOLARIZED,
    GRUVBOX_YELLOW,
    GRUVBOX_BLUE,
    GRUVBOX_GREY,
];
