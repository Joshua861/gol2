use crate::{game::Game, utils::Vec2I};
use macroquad::prelude::*;
use strum::{Display, EnumIs};

#[derive(PartialEq, Display, EnumIs)]
pub enum Tool {
    Brush,
    Line {
        start: Option<Vec2I>,
        end: Option<Vec2I>,
    },
    Selection {
        start: Option<Vec2I>,
        end: Option<Vec2I>,
    },
}

impl Game {
    pub fn handle_input(&mut self) {
        self.zooming();
        self.panning();

        let mouse_pos = self.mouse_pos();

        match &mut self.selected_tool {
            Tool::Brush => {
                if is_mouse_button_down(MouseButton::Left) {
                    self.drawing(true);
                } else if is_mouse_button_down(MouseButton::Right) {
                    self.drawing(false);
                }
            }
            Tool::Line { start, end } => {
                if is_mouse_button_pressed(MouseButton::Left) {
                    *start = Some(mouse_pos);
                    *end = Some(mouse_pos);
                } else if is_mouse_button_down(MouseButton::Left) && start.is_some() {
                    *end = Some(mouse_pos);
                } else if start.is_some() && end.is_some() {
                    if let Some(line) = self.get_brush_line() {
                        for (x, y) in line {
                            self.board.set(x, y, true);
                        }
                    }

                    self.selected_tool = Tool::Line {
                        start: None,
                        end: None,
                    }
                }
            }
            Tool::Selection { start, end } => {
                if is_mouse_button_pressed(MouseButton::Left) {
                    *start = Some(mouse_pos);
                    *end = Some(mouse_pos);
                } else if is_mouse_button_down(MouseButton::Left) && start.is_some() {
                    *end = Some(mouse_pos);
                } else if start.is_some()
                    && end.is_some()
                    && (is_key_pressed(KeyCode::Escape)
                        || is_mouse_button_pressed(MouseButton::Right))
                {
                    if let Some(line) = self.get_brush_line() {
                        for (x, y) in line {
                            self.board.set(x, y, true);
                        }
                    }

                    self.selected_tool = Tool::Selection {
                        start: None,
                        end: None,
                    }
                }
            }
        }

        if is_key_pressed(KeyCode::Space) {
            self.paused = !self.paused;
        }

        if is_key_pressed(KeyCode::C) {
            self.board.clear();
        }

        if is_key_pressed(KeyCode::N) {
            self.board.update(&self.config);
        }
    }

    fn zooming(&mut self) {
        let delta = mouse_wheel().1;

        self.camera.zoom += delta * self.config.zoom_speed * self.camera.zoom;
    }

    fn panning(&mut self) {
        if !is_mouse_button_down(MouseButton::Middle) {
            return;
        }

        let delta = mouse_delta_position();

        self.camera.offset -= (delta / self.camera.zoom) * self.config.pan_speed;
    }

    fn drawing(&mut self, to: bool) {
        let pos = self.mouse_pos();

        if let Some(last_pos) = &self.mouse_pos_last_frame {
            if self.config.brush_radius == 1 {
                self.board
                    .set_line(pos.x, pos.y, last_pos.x, last_pos.y, to);
            } else {
                self.board.draw_line(
                    pos.x as usize,
                    pos.y as usize,
                    last_pos.x as usize,
                    last_pos.y as usize,
                    self.config.brush_radius,
                    to,
                );
            }
        } else {
            if self.config.brush_radius == 1 {
                self.board.set(pos.x, pos.y, to);
            } else {
                self.board
                    .draw(pos.x as usize, pos.y as usize, self.config.brush_radius, to);
            }
        }
    }

    pub fn mouse_pos(&self) -> Vec2I {
        let cur_pos = mouse_position();
        let (x, y) = self.screen_to_board(cur_pos.0, cur_pos.1);
        Vec2I { x, y }
    }

    pub fn get_selection(&self) -> Option<Selection> {
        if let Tool::Selection { start, end } = self.selected_tool {
            let start = start?;
            let end = end?;

            let min_x = start.x.min(end.x);
            let max_x = start.x.max(end.x);
            let min_y = start.y.min(end.y);
            let max_y = start.y.max(end.y);

            let start = Vec2I::new(max_x, max_y);
            let end = Vec2I::new(min_x, min_y);

            let width = start.x - end.x + 1;
            let height = start.y - end.y + 1;

            return Some(Selection {
                start,
                end,
                width,
                height,
            });
        }

        None
    }

    // TODO: pub fn copy_selection(&mut self)
}

pub struct Selection {
    pub start: Vec2I,
    pub end: Vec2I,
    pub width: isize,
    pub height: isize,
}
