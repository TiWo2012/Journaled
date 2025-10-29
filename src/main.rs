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
}

impl NoteApp {
    fn ui(&mut self, ui: &mut egui::Ui) {
        // Title
        ui.vertical_centered(|ui| {
            ui.heading("📝 Journal Entry");
        });
        ui.add_space(10.0);

        // Editor
        self.ui_editor(ui);
        ui.add_space(10.0);

        // Save
        self.ui_save(ui);
    }

    fn ui_editor(&mut self, ui: &mut egui::Ui) {
        egui::Frame::group(ui.style())
            .inner_margin(egui::vec2(10.0, 10.0))
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Title:");
                    ui.text_edit_singleline(&mut self.note.title);
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
                            self.save_path.get_or_insert_with(|| "note.txt".into()),
                        );

                        ui.add_space(10.0);

                        if ui.button("Save Note").clicked() {
                            self.save();
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

    fn save(&self) {
        if let Some(path) = &self.save_path {
            let content = format!(
                "Title: {}\nDate: {:02}-{:02}-{}\n\n{}",
                self.note.title,
                self.note.date.day,
                self.note.date.month,
                self.note.date.year,
                self.note.content
            );

            if let Err(err) = fs::write(path, content) {
                eprintln!("Error saving note: {}", err);
            }
        }
    }
}

impl Default for NoteApp {
    fn default() -> Self {
        Self {
            note: Note::default(),
            save_path: None,
            last_save_msg: None,
        }
    }
}

impl eframe::App for NoteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui(ui);
        });
    }
}
