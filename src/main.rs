use chrono::{Datelike, Local};

mod noteapp;
use noteapp::NoteApp;

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

