use chrono::{Datelike, Local};
use eframe::egui;
use std::fs;

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

impl Note {
    fn new(title: String, content: String, date: Date) -> Self {
        Self {
            title,
            content,
            date,
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Title:");
        ui.text_edit_singleline(&mut self.title);

        ui.separator();

        ui.label("Content:");
        ui.text_edit_multiline(&mut self.content);

        ui.separator();

        ui.label("Date:");
        ui.horizontal(|ui| {
            ui.add(
                egui::DragValue::new(&mut self.date.day)
                    .range(1..=31)
                    .prefix("Day: "),
            );
            ui.add(
                egui::DragValue::new(&mut self.date.month)
                    .range(1..=12)
                    .prefix("Month: "),
            );
            ui.add(
                egui::DragValue::new(&mut self.date.year)
                    .range(1900..=9999)
                    .prefix("Year: "),
            );
        });
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
        self.note.ui(ui);
        ui.separator();

        ui.text_edit_singleline(self.save_path.get_or_insert_with(|| format!("note{}_{}_{}.txt", self.note.date.year, self.note.date.month, self.note.date.day)));

        if ui.button("Save Note").clicked() {
            self.save();
            self.last_save_msg = Some("Note saved successfully!".into());
        }

        if let Some(msg) = &self.last_save_msg {
            ui.colored_label(egui::Color32::GREEN, msg);
        }
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
            } else {
                println!("✅ Saved note to {}", path);
            }
        }
    }
}

impl Default for NoteApp {
    fn default() -> Self {
        Self {
            note: Note::new("New Note".into(), "".into(), Date::default()),
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
