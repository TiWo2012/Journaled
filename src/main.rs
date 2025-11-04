use chrono::{Datelike, Local};
use serde::{Deserialize, Serialize};

mod noteapp;
use noteapp::NoteApp;

// -----------------------------
// Entry Point
// -----------------------------
fn main() -> eframe::Result<()> {
    let mut options = eframe::NativeOptions::default();
    options.renderer = eframe::Renderer::Glow;
    options.run_and_return = false;
    eframe::run_native(
        "Note Taking App",
        options,
        Box::new(|_cc| Ok(Box::new(NoteApp::default()))),
    )
}

// -----------------------------
// Data Models
// -----------------------------
#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_default() {
        let date = Date::default();
        assert_eq!(date.day, Local::now().day());
        assert_eq!(date.month, Local::now().month());
        assert_eq!(date.year, Local::now().year() as u32);
    }

    #[test]
    fn test_note_default() {
        let note = Note::default();
        assert_eq!(note.title, "New Note");
        assert_eq!(note.content, "");
        assert_eq!(note.date.day, Local::now().day());
        assert_eq!(note.date.month, Local::now().month());
        assert_eq!(note.date.year, Local::now().year() as u32);
    }

    #[test]
    fn test_note_title() {
        let note = Note::default();
        assert_eq!(note.title, "New Note");
    }

    #[test]
    fn test_note_content() {
        let note = Note::default();
        assert_eq!(note.content, "");
    }

    #[test]
    fn test_note_date() {
        let note = Note::default();
        assert_eq!(note.date.day, Local::now().day());
        assert_eq!(note.date.month, Local::now().month());
        assert_eq!(note.date.year, Local::now().year() as u32);
    }

    #[test]
    fn test_note_title_length() {
        let note = Note::default();
        assert!(note.title.len() <= 100);
    }

    #[test]
    fn test_note_content_length() {
        let note = Note::default();
        assert!(note.content.len() <= 1000);
    }

    #[test]
    fn test_note_title_not_empty() {
        let note = Note::default();
        assert!(!note.title.is_empty());
    }

    #[test]
    fn test_note_title_not_whitespace() {
        let note = Note::default();
        assert!(!note.title.chars().all(char::is_whitespace));
    }

    #[test]
    fn test_note_title_not_whitespace_only() {
        let note = Note::default();
        assert!(!note.title.chars().all(char::is_whitespace));
    }
}
