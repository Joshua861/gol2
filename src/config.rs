use dirs::data_dir;
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, read_to_string, File};
use std::io::Write;

use crate::{
    color_schemes::{ColorScheme, DARK},
    utils::GColor,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub width: usize,
    pub height: usize,
    pub bg_color: GColor,
    pub dead_color: GColor,
    pub alive_color: GColor,
    pub hot_color: GColor,
    pub text_color: GColor,
    pub line_color: GColor,
    pub tile_size: f32,
    pub heat_intensity: f32,
    pub highlight_color: GColor,
    pub zoom_speed: f32,
    pub soft_heat: bool,
    pub soft_heat_amount: u8,
    pub enable_heat: bool,
    pub color_scheme: ColorScheme,
    pub color_scheme_last_frame: ColorScheme,
    pub pan_speed: f32,
    pub simulation_speed: usize,
}

impl Config {
    pub fn save(&self) {
        let text = serde_json::to_string(&self).unwrap();
        let path = Self::path();
        create_dir_all(Self::folder_path()).unwrap();
        let mut output = File::create(&path).unwrap();

        write!(output, "{}", text).unwrap();
    }

    fn path() -> String {
        format!("{}/gol2/config.json", data_dir().unwrap().display())
    }

    fn folder_path() -> String {
        format!("{}/gol2", data_dir().unwrap().display())
    }

    pub fn load() -> Self {
        let path = Self::path();

        if let Ok(text) = read_to_string(path) {
            serde_json::from_str(&text).unwrap_or_default()
        } else {
            Self::default()
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            width: 192,
            height: 108,

            tile_size: 16.0,
            heat_intensity: 0.5,
            zoom_speed: 0.1,
            soft_heat_amount: 50,
            simulation_speed: 1,
            pan_speed: 100.0,

            bg_color: GColor::from_hex(0x002B36),
            dead_color: GColor::from_hex(0x073642),
            alive_color: GColor::from_hex(0xFDF6E3),
            hot_color: GColor::from_hex(0x586E75),
            text_color: GColor::from_hex(0xFFFFFF),
            highlight_color: GColor::from_rgba(255, 255, 255, 50),
            line_color: GColor::from_rgba(255, 255, 255, 50),

            color_scheme: DARK,
            color_scheme_last_frame: DARK,

            soft_heat: false,
            enable_heat: true,
        }
    }
}
