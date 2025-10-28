use eframe::egui;

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
        Self { day: 1, month: 1, year: 2024 }
    }
}

struct Note {
    title: String,
    content: String,
    date: Date,
}

impl Note {
    fn new(title: String, content: String, date: Date) -> Self {
        Self { title, content, date }
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
            ui.add(egui::DragValue::new(&mut self.date.day).range(1..=31).prefix("Day: "));
            ui.add(egui::DragValue::new(&mut self.date.month).range(1..=12).prefix("Month: "));
            ui.add(egui::DragValue::new(&mut self.date.year).range(1900..=2100).prefix("Year: "));
        });
    }
}

// -----------------------------
// Application
// -----------------------------
struct NoteApp {
    note: Note,
}

impl Default for NoteApp {
    fn default() -> Self {
        Self {
            note: Note::new("New Note".into(), "".into(), Date::default()),
        }
    }
}

impl eframe::App for NoteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Simple Note App");
            ui.separator();
            self.note.ui(ui);
        });
    }
}
