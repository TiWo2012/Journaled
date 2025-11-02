use chrono::{Datelike, Local};
use eframe::egui;
use std::fs;

// -----------------------------
// Entry Point
// -----------------------------
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Note Taking App",
        options,
        Box::new(|_cc| Ok(Box::new(NoteApp::default()))),
    )
}

// -----------------------------
// Data Models
// -----------------------------
struct Date {
    day: u32,
    month: u32,
    year: u32,
}

impl Default for Date {
    fn default() -> Self {
        let now = Local::now();

        Self {
            day: now.day(),
            month: now.month(),
            year: now.year() as u32,
        }
    }
}

struct Note {
    title: String,
    content: String,
    date: Date,
}

impl Default for Note {
    fn default() -> Self {
        Self {
            title: "New Note".into(),
            content: "".into(),
            date: Date::default(),
        }
    }
}

// -----------------------------
// Application
// -----------------------------
struct NoteApp {
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
        ui.horizontal(|ui| {
            // left side (file browser)
            ui.vertical(|ui| {
                // size settings for the explorer
                ui.set_min_width(150.0);
                ui.set_max_width(200.0);

                // the explorer itself
                self.ui_file_browser(ui);
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

    fn ui_file_browser(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
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

            egui::ScrollArea::vertical()
                .auto_shrink([true; 2])
                .stick_to_bottom(false)
                .max_width(200.0)
                .show(ui, |ui| {
                    let entries =
                        fs::read_dir("notes/").unwrap_or_else(|_| fs::read_dir(".").unwrap());

                    for entry in entries.flatten() {
                        let file_name = entry.file_name().into_string().unwrap_or_default();

                        if !self.search_query.is_empty()
                            && !file_name
                                .to_lowercase()
                                .contains(&self.search_query.to_lowercase())
                        {
                            continue;
                        }
                    }

                    // TODO: Fix the scroplable area
                    let entries =
                        fs::read_dir("notes/").unwrap_or_else(|_| fs::read_dir(".").unwrap());
                    for entry in entries.flatten() {
                        let file_name = entry.file_name().into_string().unwrap_or_default();

                        if !self.search_query.is_empty()
                            && !file_name
                                .to_lowercase()
                                .contains(&self.search_query.to_lowercase())
                        {
                            continue;
                        }

                        ui.horizontal(|ui| {
                            ui.group(|ui| {
                                if ui.button(&file_name).clicked() {
                                    self.load_note(&file_name);
                                }
                                if ui.button("❌").clicked() {
                                    let _ = fs::remove_file(format!("notes/{}", file_name));
                                }
                            });
                        });
                    }
                });

            ui.add_space(10.0);

            // ➕ New Note button
            self.ui_new_note(ui);
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
                        self.save_path = Some(format!("{}.txt", self.note.title));
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
                                .get_or_insert_with(|| format!("{}.txt", self.note.title).into()),
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
            let content = format!(
                "Title: {}\nDate: {:02}-{:02}-{}\n\n{}",
                self.note.title,
                self.note.date.day,
                self.note.date.month,
                self.note.date.year,
                self.note.content
            );

            fs::DirBuilder::new()
                .recursive(true)
                .create("notes")
                .unwrap_or(());
            let actual_save_path = format!("notes/{}", path);

            if let Err(err) = fs::write(actual_save_path, content) {
                eprintln!("Error saving note: {}", err);
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
        if let Ok(content) = fs::read_to_string(&path) {
            let mut lines = content.lines();
            if let Some(title_line) = lines.next() {
                if title_line.starts_with("Title: ") {
                    self.note.title = title_line[7..].to_string();
                }
            }
            if let Some(date_line) = lines.next() {
                if date_line.starts_with("Date: ") {
                    let date_str = &date_line[6..];
                    let parts: Vec<&str> = date_str.split('-').collect();
                    if parts.len() == 3 {
                        if let (Ok(day), Ok(month), Ok(year)) = (
                            parts[0].parse::<u32>(),
                            parts[1].parse::<u32>(),
                            parts[2].parse::<u32>(),
                        ) {
                            self.note.date = Date { day, month, year };
                        }
                    }
                }
            }
            self.note.content = lines.collect::<Vec<&str>>().join("\n");
        }
    }
}

impl eframe::App for NoteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // keyboard shortcuts
        if ctx.input(|i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl) {
            self.save_note();
        }
        if ctx.input(|i| i.key_pressed(egui::Key::N) && i.modifiers.ctrl) {
            self.new_note();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui(ui);
        });
    }
}
