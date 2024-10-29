use crate::{game::Game, input::Tool, utils::Vec2I};
use clipline::Clipline;
use macroquad::{
    input::show_mouse, math::Vec2, miniquad::window::screen_size, shapes::draw_rectangle,
    window::clear_background,
};

pub struct Camera {
    pub zoom: f32,
    pub offset: Vec2,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            zoom: 1.,
            offset: Vec2::ZERO,
        }
    }
}

impl Game {
    pub fn draw(&self) {
        clear_background(self.config.bg_color.to_mq());

        if !self.config.enable_heat {
            let (x, y) = self.board_to_screen(0, 0);
            let (w, h) = self.board_wh_screen();
            draw_rectangle(x, y, w, h, self.config.dead_color.to_mq());
        }

        for y in 0..self.board.height() {
            for x in 0..self.board.width() {
                let tile = self.board.get(x as isize, y as isize);
                if self.config.enable_heat {
                    let color = if tile.alive() {
                        self.config.alive_color
                    } else if tile.heat() != 0 {
                        self.config.hot_color.blend(
                            &self.config.dead_color,
                            1. - (tile.heat() as f32 / 255.) * self.config.heat_intensity,
                        )
                    } else {
                        self.config.dead_color
                    };

                    let (dx, dy) = self.board_to_screen(x as isize, y as isize);
                    let s = self.tile_size();

                    draw_rectangle(dx, dy, s, s, color.to_mq());
                } else if tile.alive() {
                    let (dx, dy) = self.board_to_screen(x as isize, y as isize);
                    let s = self.tile_size();

                    draw_rectangle(dx, dy, s, s, self.config.alive_color.to_mq());
                }
            }
        }

        let _ = self.draw_line();

        {
            let Vec2I { x, y } = self.mouse_pos();
            self.draw_highlight(x, y);
        }
    }

    fn draw_highlight(&self, x: isize, y: isize) {
        if !self.board.is_inside(x, y) {
            show_mouse(true);
            return;
        }

        show_mouse(false);

        let (x, y) = self.board_to_screen(x, y);
        let s = self.tile_size();

        draw_rectangle(x, y, s, s, self.config.highlight_color.to_mq());
    }

    pub fn get_brush_line(&self) -> Option<Clipline> {
        if let Tool::Line { start, end } = self.selected_tool {
            if let Some(start) = start {
                if let Some(end) = end {
                    return Some(Clipline::new(
                        ((start.x, start.y), (end.x, end.y)),
                        (
                            (0, 0),
                            (
                                (self.board.width() as isize) - 1,
                                (self.board.height() as isize) - 1,
                            ),
                        ),
                    )?);
                }
            }
        }

        None
    }

    fn draw_line(&self) {
        if let Some(line) = self.get_brush_line() {
            for (x, y) in line {
                let (x, y) = self.board_to_screen(x, y);

                draw_rectangle(
                    x,
                    y,
                    self.tile_size(),
                    self.tile_size(),
                    self.config.line_color.to_mq(),
                );
            }
        }
    }

    pub fn tile_size(&self) -> f32 {
        self.camera.zoom * self.config.tile_size
    }

    pub fn board_wh(&self) -> (usize, usize) {
        (self.board.width(), self.board.height())
    }

    pub fn board_wh_screen(&self) -> (f32, f32) {
        let (bw, bh) = self.board_wh();
        let s = self.tile_size();
        let (bw, bh) = (bw as f32 * s, bh as f32 * s);

        (bw, bh)
    }

    pub fn board_to_screen(&self, x: isize, y: isize) -> (f32, f32) {
        let c = &self.camera;
        let s = self.tile_size();
        let (sw, sh) = screen_size();
        let (bw, bh) = self.board_wh_screen();

        let mut x = x as f32;
        let mut y = y as f32;

        x += c.offset.x;
        y += c.offset.y;

        x *= s;
        y *= s;

        x += (sw - bw) / 2.;
        y += (sh - bh) / 2.;

        (x, y)
    }

    pub fn screen_to_board(&self, mut x: f32, mut y: f32) -> (isize, isize) {
        let c = &self.camera;
        let s = self.tile_size();
        let (sw, sh) = screen_size();
        let (bw, bh) = self.board_wh_screen();

        x -= (sw - bw) / 2.;
        y -= (sh - bh) / 2.;

        x /= s;
        y /= s;

        x -= c.offset.x;
        y -= c.offset.y;

        let x = x as isize;
        let y = y as isize;

        (x, y)
    }
}
