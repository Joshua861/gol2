use crate::{
    board::Board,
    color_schemes::THEMES,
    game::Game,
    input::Tool,
    rules::{CONWAY, FALLING_STARS, MAZE, MAZE_MICE, RULES},
    utils::GColor,
};
use egui_macroquad::{
    egui::{self, ComboBox, Pos2, RichText, Ui},
    ui,
};

#[derive(Default)]
pub struct UiState {
    save_name: String,
    load_name: String,
}

impl Game {
    pub fn update_ui(&mut self) {
        ui(|ctx| {
            egui::Window::new("Settings")
                .default_open(false)
                .default_pos(Pos2::new(50., 50.))
                .show(ctx, |ui| {
                    ui.label(RichText::new("Colors").text_style(egui::TextStyle::Heading));

                    color_picker(ui, &mut self.config.text_color, "Text color");
                    color_picker(ui, &mut self.config.bg_color, "Background color");
                    color_picker(ui, &mut self.config.alive_color, "Alive color");
                    if self.config.enable_heat {
                        color_picker(ui, &mut self.config.hot_color, "Hot color");
                    }
                    color_picker(ui, &mut self.config.dead_color, "Dead color");
                    color_picker(ui, &mut self.config.highlight_color, "Highlight color");

                    ui.add_space(8.);

                    {
                        let _ = ComboBox::from_label("Color Scheme")
                            .selected_text(self.config.color_scheme.name.to_string())
                            .show_ui(ui, |ui| {
                                for theme in THEMES {
                                    ui.selectable_value(
                                        &mut self.config.color_scheme,
                                        theme.clone(),
                                        theme.name.to_string(),
                                    );
                                }
                            });
                    }

                    ui.add_space(16.);
                    ui.label(RichText::new("Heat").text_style(egui::TextStyle::Heading));

                    ui.checkbox(
                        &mut self.config.enable_heat,
                        "Enable heat (reduces performance)",
                    );

                    if self.config.enable_heat {
                        ui.checkbox(&mut self.config.soft_heat, "Soft heat (default off)");

                        if self.config.soft_heat {
                            let mut heat_f32 = self.config.soft_heat_amount as f32;
                            ui.add(
                                egui::Slider::new(&mut heat_f32, 0.0..=255.)
                                    .text("Soft heat amount"),
                            );
                            self.config.soft_heat_amount = heat_f32 as u8;
                        }

                        ui.add(
                            egui::Slider::new(&mut self.config.heat_intensity, 0.0..=1.0)
                                .text("Heat intensity (default 0.5)"),
                        );
                    }

                    ui.add_space(16.);
                    ui.label(RichText::new("Board").text_style(egui::TextStyle::Heading));

                    usize_slider(ui, &mut self.config.width, 16, 500, "Width (default 192)");
                    usize_slider(ui, &mut self.config.height, 16, 500, "Height (default 108)");

                    ui.add_space(4.);

                    if ui.button("Create new board").clicked() {
                        self.board = Board::new(self.config.width, self.config.height);
                    }

                    ui.add_space(16.);
                    ui.label(RichText::new("Tools").text_style(egui::TextStyle::Heading));

                    if ui.button("Clear board (C)").clicked() {
                        self.board.clear();
                    }

                    if ui.button("Randomize board").clicked() {
                        self.board.randomize();
                    }

                    ui.add_space(8.);

                    if ui.button("Save config").clicked() {
                        self.config.save();
                    }
                    ui.label("Config auto saves every 300 frames.");

                    ui.add_space(16.);

                    ui.label(RichText::new("Save board").size(14.));

                    ui.label("Board save name:");
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut self.ui_state.save_name);
                        if ui.button("Save").clicked() && !self.ui_state.save_name.is_empty() {
                            self.save_board(self.ui_state.save_name.clone()).unwrap();
                            self.ui_state.save_name = "".into();
                        }
                    });

                    ui.add_space(8.);
                    ui.label(RichText::new("Load board").size(14.));
                    ui.horizontal(|ui| {
                        let _ = ComboBox::from_label("")
                            .selected_text(self.ui_state.load_name.to_string())
                            .show_ui(ui, |ui| {
                                for save_name in &self.saves {
                                    ui.selectable_value(
                                        &mut self.ui_state.load_name,
                                        save_name.clone(),
                                        save_name,
                                    );
                                }
                            });

                        if ui.button("Load").clicked() && !self.ui_state.load_name.is_empty() {
                            self.load_board(self.ui_state.load_name.clone()).unwrap();
                        }
                    });

                    ui.add_space(16.);
                    ui.label(RichText::new("Other").text_style(egui::TextStyle::Heading));

                    ui.add(
                        egui::Slider::new(&mut self.config.zoom_speed, 0.01..=0.3)
                            .text("Zoom speed (default 0.1)"),
                    );

                    ui.add(
                        egui::Slider::new(&mut self.config.pan_speed, 1.0..=500.0)
                            .text("Pan speed (default 100)"),
                    );

                    usize_slider(
                        ui,
                        &mut self.config.simulation_speed,
                        1,
                        10,
                        "Simulation speed (default 1)",
                    );

                    ui.add_space(8.);

                    let _ = ComboBox::from_label("Rule")
                        .selected_text("Select rule")
                        .show_ui(ui, |ui| {
                            for (name, rule) in RULES {
                                ui.selectable_value(&mut self.config.rule, *rule, *name);
                            }
                        });

                    ui.add_space(16.);

                    let _ = ComboBox::from_label(RichText::new("Tool").size(14.))
                        .selected_text(self.selected_tool.to_string())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.selected_tool, Tool::Brush, "Brush");
                            ui.selectable_value(
                                &mut self.selected_tool,
                                Tool::Line {
                                    start: None,
                                    end: None,
                                },
                                "Line",
                            );
                            ui.selectable_value(
                                &mut self.selected_tool,
                                Tool::Selection {
                                    start: None,
                                    end: None,
                                },
                                "Selection",
                            );
                        });

                    if self.selected_tool.is_brush() {
                        usize_slider(ui, &mut self.config.brush_radius, 1, 10, "Brush radius");
                    }
                });
        });

        egui_macroquad::draw();
    }
}


fn color_picker(ui: &mut Ui, color: &mut GColor, label: &str) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.add_space(8.0);
        let mut color_arr = color.to_rgba_arr();
        ui.color_edit_button_rgba_unmultiplied(&mut color_arr);
        *color = GColor::from_rgba_arr(color_arr);
    });
}

fn usize_slider(ui: &mut Ui, value: &mut usize, min: usize, max: usize, label: &str) {
    let mut value_f32 = *value as f32;
    ui.add(egui::Slider::new(&mut value_f32, min as f32..=max as f32).text(label));
    *value = value_f32 as usize;
}
