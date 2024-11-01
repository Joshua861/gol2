use anyhow::Result;
use dirs::video_dir;
use gif::{Encoder, Frame, Repeat};
use image::{EncodableLayout, RgbaImage};
use std::fs::{create_dir_all, File};
use std::path::Path;

use crate::{board::Board, config::Config};

pub struct Recording {
    name: String,
    num_frames: usize,
    frames: Vec<RgbaImage>,
    upscale: usize,
    width: usize,
    height: usize,
    board: Board,
    config: Config,
    frame_rate: usize,
}

impl Recording {
    pub fn new(
        board: &Board,
        config: &Config,
        name: String,
        num_frames: usize,
        upscale: usize,
        frame_rate: usize,
    ) -> Self {
        Self {
            name,
            num_frames,
            frames: vec![],
            upscale,
            frame_rate,
            height: board.height(),
            width: board.width(),
            board: board.clone(),
            config: config.clone(),
        }
    }

    pub fn render(mut self) -> Self {
        for _ in 0..=self.num_frames {
            self.render_frame();
        }

        self
    }

    pub fn encode(&self) -> Result<(), Box<dyn std::error::Error>> {
        create_dir_all(Self::recording_folder()).unwrap();

        let frames = self.frames.clone();

        if frames.is_empty() {
            return Err("No frames to encode".into());
        }

        let file = File::create(self.path())?;
        let mut encoder = Encoder::new(
            file,
            frames[0].width() as u16,
            frames[0].height() as u16,
            &[],
        )?;

        encoder.set_repeat(Repeat::Infinite)?;

        let delay = 100 / self.frame_rate;

        for image in frames {
            let mut pixels: Vec<u8> = image.as_raw().to_vec();

            let mut frame =
                Frame::from_rgba_speed(image.width() as u16, image.height() as u16, &mut pixels, 1);

            frame.delay = delay as u16;
            encoder.write_frame(&frame)?;
        }

        Ok(())
    }

    fn recording_folder() -> String {
        format!("{}/gol2", video_dir().unwrap().display())
    }

    pub fn path_from_name(name: String) -> String {
        format!("{}/{}.gif", Self::recording_folder(), name)
    }

    fn path(&self) -> String {
        Self::path_from_name(self.name.clone())
    }

    fn i_width(&self) -> u32 {
        (self.width * self.upscale) as u32
    }

    fn i_height(&self) -> u32 {
        (self.height * self.upscale) as u32
    }

    fn render_frame(&mut self) {
        let mut buf = RgbaImage::new(self.i_width(), self.i_height());
        let board = &mut self.board;
        let config = &self.config;

        for y in 0..self.height {
            for x in 0..self.width {
                let tile = board.get(x as isize, y as isize);

                let color = if tile.alive {
                    config.alive_color
                } else if tile.heat() != 0 && config.enable_heat {
                    config.hot_color.blend(
                        &config.dead_color,
                        1. - (tile.heat() as f32 / 255.) * self.config.heat_intensity,
                    )
                } else {
                    config.dead_color
                };

                for dx in 0..self.upscale {
                    for dy in 0..self.upscale {
                        buf.put_pixel(
                            (x * self.upscale + dx) as u32,
                            (y * self.upscale + dy) as u32,
                            color.to_img(),
                        );
                    }
                }
            }
        }

        self.frames.push(buf);
        self.board.update(&self.config);
    }
    // for y in 0..self.board.height() {
    //     for x in 0..self.board.width() {
    //         let tile = self.board.get(x as isize, y as isize);
    //         if self.config.enable_heat {
    //             let color = if tile.alive() {
    //                 self.config.alive_color
    //             } else if tile.heat() != 0 {
    //                 self.config.hot_color.blend(
    //                     &self.config.dead_color,
    //                     1. - (tile.heat() as f32 / 255.) * self.config.heat_intensity,
    //                 )
    //             } else {
    //                 self.config.dead_color
    //             };
    //
    //             let (dx, dy) = self.board_to_screen(x as isize, y as isize);
    //             let s = self.tile_size();
    //
    //             draw_rectangle(dx, dy, s, s, color.to_mq());
    //         } else if tile.alive() {
    //             let (dx, dy) = self.board_to_screen(x as isize, y as isize);
    //             let s = self.tile_size();
    //
    //             draw_rectangle(dx, dy, s, s, self.config.alive_color.to_mq());
    //         }
    //     }
    // }
}
