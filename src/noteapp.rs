use crate::Note;
use eframe::egui;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
#[derive(Serialize, Deserialize)]
pub struct NoteApp {
    note: Note,
    save_path: Option<String>,
    last_save_msg: Option<String>,
    search_query: String,
}

impl Default for NoteApp {
    fn default() -> Self {
        Self {
            note: Note::default(),
            save_path: None,
            last_save_msg: None,
            search_query: String::new(),
        }
    }
}

impl NoteApp {
    fn ui(&mut self, ui: &mut egui::Ui) {
        // 🧭 Get remaining height for the scrollable list
        let available_height = ui.available_height();
        ui.horizontal(|ui| {
            // left side (file browser)
            ui.vertical(|ui| {
                // size settings for the explorer
                ui.set_min_width(190.0);
                ui.set_max_width(200.0);

                // the explorer itself
                self.ui_file_browser(ui, available_height);
            });

            ui.separator();

            // right side (editor + save)
            ui.vertical_centered(|ui| {
                // Title
                ui.heading("📝 Journal Entry");
                ui.add_space(10.0);

                // Editor
                self.ui_editor(ui);
                ui.add_space(10.0);

                // Save
                self.ui_save(ui);
            });
        });
    }

    fn ui_file_browser(&mut self, ui: &mut egui::Ui, available_height: f32) {
        ui.vertical_centered(|ui| {
            ui.heading("📁 Notes");
            ui.add_space(5.0);
            ui.label("Search:");

            ui.horizontal_centered(|ui| {
                ui.text_edit_singleline(&mut self.search_query);
                if ui.button("❌").on_hover_text("Clear search").clicked() {
                    self.search_query.clear();
                }
            });
        });

        ui.separator();
        ui.add_space(5.0);

        // Note: Please don't fail me again scroll line
        ui.set_min_height(available_height - 175.0);
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .stick_to_bottom(false)
            .show(ui, |ui| {
                // List notes
                if let Ok(entries) = fs::read_dir("notes/") {
                    for entry in entries.flatten() {
                        let file_name = entry.file_name().into_string().unwrap_or_default();

                        // Apply search filter
                        if !self.search_query.is_empty()
                            && !file_name
                                .to_lowercase()
                                .contains(&self.search_query.to_lowercase())
                        {
                            continue;
                        }

                        // Remove file extension for display
                        let display_name = file_name.replace(".json", "");

                        ui.horizontal(|ui| {
                            if ui.button(&display_name).clicked() {
                                self.load_note(&file_name);
                            }

                            if ui.button("❌").clicked() {
                                let _ = fs::remove_file(format!("notes/{}", file_name));
                            }
                        });
                    }
                } else {
                    ui.label("No notes found.");
                }
            });

        ui.add_space(10.0);
        ui.vertical_centered(|ui| {
            if ui.button("➕ New Note").clicked() {
                self.ui_new_note(ui);
            }
        });
    }

    fn ui_editor(&mut self, ui: &mut egui::Ui) {
        egui::Frame::group(ui.style())
            .inner_margin(egui::vec2(10.0, 10.0))
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Title:");
                    let response = ui.text_edit_singleline(&mut self.note.title);

                    // 💡 Auto-update filename when the title changes
                    if response.changed() {
                        // Only auto-update if save_path wasn’t set manually
                        self.save_path = Some(format!("{}.json", self.note.title));
                    }
                });

                ui.separator();

                ui.vertical_centered(|ui| {
                    ui.label("Content:");
                    ui.add(
                        egui::TextEdit::multiline(&mut self.note.content)
                            .desired_rows(15)
                            .desired_width(f32::INFINITY),
                    );
                });

                ui.separator();

                ui.vertical_centered(|ui| {
                    ui.label(format!(
                        "📅 Date: {:02}-{:02}-{}",
                        self.note.date.day, self.note.date.month, self.note.date.year
                    ));
                });
            });
    }

    fn ui_save(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(20.0); // top padding
            ui.set_max_width(400.0); // limit width like a document column

            egui::Frame::group(ui.style())
                .inner_margin(egui::vec2(15.0, 12.0))
                .stroke(egui::Stroke::new(1.0, egui::Color32::LIGHT_BLUE))
                .corner_radius(egui::CornerRadius::same(10))
                .shadow(egui::epaint::Shadow {
                    offset: [0, 2],
                    blur: 5,
                    color: egui::Color32::from_black_alpha(50),
                    spread: 0,
                })
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label("💾 Save Path:");
                        ui.text_edit_singleline(
                            self.save_path
                                .get_or_insert_with(|| format!("{}.json", self.note.title).into()),
                        );

                        ui.add_space(10.0);

                        if ui.button("Save Note").clicked() {
                            self.save_note();
                            self.last_save_msg = Some("Note saved successfully!".into());
                        }

                        if let Some(msg) = &self.last_save_msg {
                            ui.add_space(8.0);
                            ui.colored_label(egui::Color32::LIGHT_GREEN, msg);
                        }
                    });
                });
        });
    }

    fn save_note(&self) {
        if let Some(path) = &self.save_path {
            fs::create_dir_all("notes").unwrap_or(());
            let actual_save_path = format!("notes/{}", path);

            // Serialize the Note struct into pretty JSON
            match serde_json::to_string_pretty(&self.note) {
                Ok(json) => {
                    if let Err(err) = fs::write(&actual_save_path, json) {
                        eprintln!("Error saving note: {}", err);
                    }
                }
                Err(err) => eprintln!("Error serializing note: {}", err),
            }
        }
    }

    fn ui_new_note(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            if ui.button("➕ New Note").clicked() {
                self.new_note();
            }
        });
    }

    fn new_note(&mut self) {
        self.note = Note::default();
        self.save_path = None;
        self.last_save_msg = None;
    }

    fn load_note(&mut self, file_name: &str) {
        let path = format!("notes/{}", file_name);
        if let Ok(json_str) = fs::read_to_string(&path) {
            match serde_json::from_str::<Note>(&json_str) {
                Ok(note) => self.note = note,
                Err(err) => eprintln!("Error loading note: {}", err),
            }
        }
    }

    fn keyboard_shortcuts(&mut self, ctx: &egui::Context) {
        if ctx.input(|i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl) {
            self.save_note();
        }

        if ctx.input(|i| i.key_pressed(egui::Key::N) && i.modifiers.ctrl) {
            self.new_note();
        }

        // TODO: implement actual tabs
        if ctx.input(|i| i.key_pressed(egui::Key::W) && i.modifiers.ctrl) {
            self.new_note();
        }

        if ctx.input(|i| i.key_pressed(egui::Key::Q) && i.modifiers.ctrl) {
            std::process::exit(0);
        }
    }
}

impl eframe::App for NoteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // keyboard shortcuts
        self.keyboard_shortcuts(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui(ui);
        });
    }
}
