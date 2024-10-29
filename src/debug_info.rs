use macroquad::{miniquad::window::screen_size, text::draw_multiline_text};

use crate::game::Game;

impl Game {
    pub fn render_debug_info(&self) {
        let (_, mut y) = screen_size();
        y -= 32. * 2.;

        draw_multiline_text(
            &self.debug_info(),
            12.,
            y,
            32.,
            Some(1.2),
            self.config.text_color.to_mq(),
        );
    }

    pub fn fps(&self) -> f64 {
        self.fps_ticker.avg()
    }

    fn debug_info(&self) -> String {
        let mut info = format!("FPS: {:.2}\nIteration: {}\n", self.fps(), self.iter_count);

        if self.paused {
            info = format!("{}\nPaused", info);
        }

        info
    }
}
