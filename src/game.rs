use crate::{
    board::Board, config::Config, input::Tool, notifications::NotificationState, notify_info,
    rendering::Camera, ui::UiState, utils::Vec2I,
};
use fps_ticker::Fps;
use log::info;
use macroquad::prelude::*;
use std::fs::{self, create_dir_all};

pub struct Game {
    pub board: Board,
    pub config: Config,
    pub iter_count: u64,
    pub camera: Camera,
    pub fps_ticker: Fps,
    pub mouse_pos_last_frame: Option<Vec2I>,
    pub paused: bool,
    pub frame_counter: u64,
    pub ui_state: UiState,
    pub saves: Vec<String>,
    pub selected_tool: Tool,
    pub notifications: NotificationState,
}

impl Game {
    pub fn new() -> Self {
        let config = Config::load();

        Self {
            board: Board::new(config.width, config.height),
            config,
            iter_count: 0,
            camera: Camera::default(),
            fps_ticker: Fps::default(),
            mouse_pos_last_frame: None,
            paused: false,
            frame_counter: 0,
            ui_state: UiState::default(),
            saves: Self::get_saves(),
            selected_tool: Tool::Brush,
            notifications: NotificationState::new(),
        }
    }

    pub async fn run(&mut self) {
        self.board.randomize();
        self.apply_color_scheme();

        loop {
            if self.frame_counter % 300 == 0 {
                self.config.save();
                notify_info!(self, "Saved config.");
            }

            if !self.paused {
                for _ in 0..self.config.simulation_speed {
                    self.iter_count += 1;
                    self.board.update(&self.config);
                }
            }

            self.handle_input();
            self.draw();
            self.fps_ticker.tick();
            self.render_debug_info();
            self.update_ui();
            self.notifications.tick();
            self.notifications.draw(&self.config);

            if self.config.color_scheme_last_frame != self.config.color_scheme {
                self.apply_color_scheme();
            }

            self.mouse_pos_last_frame = Some(self.mouse_pos());
            self.config.color_scheme_last_frame = self.config.color_scheme.clone();
            self.frame_counter += 1;

            next_frame().await;
        }
    }

    pub fn reload_saves(&mut self) {
        self.saves = Self::get_saves();
    }

    pub fn get_saves() -> Vec<String> {
        create_dir_all(Board::saves_dir()).unwrap();
        let items = fs::read_dir(Board::saves_dir()).unwrap();

        items
            .map(|i| {
                i.unwrap()
                    .file_name()
                    .to_str()
                    .unwrap()
                    .to_string()
                    .replace(".json", "")
            })
            .collect()
    }

    pub fn save_board(&mut self, name: String) -> Result<(), String> {
        let saves_path = Board::saves_dir();
        fs::create_dir_all(&saves_path).map_err(|e| e.to_string())?;

        let path = format!("{}/{}.json", saves_path, name);
        let text = serde_json::to_string(&self.board).map_err(|e| e.to_string())?;
        fs::write(&path, text).map_err(|e| e.to_string())?;

        self.reload_saves();

        notify_info!(self, "Saved board to {}", path);
        Ok(())
    }

    pub fn load_board(&mut self, name: String) -> Result<(), String> {
        let path = format!("{}/{}.json", Board::saves_dir(), name);
        println!("{}", path);
        let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let board: Board = serde_json::from_str(&text).map_err(|e| e.to_string())?;

        self.board = board;

        notify_info!(self, "Loaded board from {}", path);

        Ok(())
    }
}
