use macroquad::{
    text::{draw_text, draw_text_ex, TextParams},
    time::get_frame_time,
};
use strum::Display;

use crate::config::Config;

pub struct NotificationState {
    notifications: Vec<Notification>,
    timer: f32,
}

#[derive(Display)]
enum NotificationType {
    Info,
    Warning,
    Error,
}

pub struct Notification {
    type_: NotificationType,
    text: String,
}

impl NotificationState {
    pub fn new() -> Self {
        Self {
            notifications: vec![],
            timer: 0.,
        }
    }

    pub fn tick(&mut self) {
        if !self.notifications.is_empty() {
            self.timer += get_frame_time();

            if self.timer > 5. {
                self.notifications.remove(self.notifications.len() - 1);
                self.timer = 0.;
            }
        }
    }

    pub fn add(&mut self, type_: NotificationType, text: &str) {
        if self.notifications.is_empty() {
            self.timer = 0.;
        }

        self.notifications.insert(
            0,
            Notification {
                type_,
                text: text.to_string(),
            },
        )
    }

    pub fn draw(&self, config: &Config) {
        let mut y = 40.;

        for notification in &self.notifications {
            draw_text_ex(
                &format!(
                    "[{}]: {}",
                    notification.type_.to_string().to_uppercase(),
                    notification.text
                ),
                10.,
                y,
                TextParams {
                    color: config.text_color.to_mq(),
                    font_size: 24,
                    ..Default::default()
                },
            );

            y += 30.;
        }
    }

    pub fn info(&mut self, text: &str) {
        self.add(NotificationType::Info, text);
    }

    pub fn warning(&mut self, text: &str) {
        self.add(NotificationType::Warning, text);
    }

    pub fn error(&mut self, text: &str) {
        self.add(NotificationType::Error, text);
    }
}
