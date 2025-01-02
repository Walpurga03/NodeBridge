pub use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Clear},
    Frame,
};
pub use chrono::{DateTime, Utc, TimeZone};
pub use crate::ui::Tab; 

/// Gemeinsame Funktion für das Anzeigen von Fehlermeldungen
#[allow(dead_code)]
pub fn show_error(message: &str) {
    eprintln!("🔴 Fehler: {}", message);
}

/// Gemeinsame Funktion für das Anzeigen von Erfolgsmeldungen
#[allow(dead_code)]
pub fn show_success(message: &str) {
    println!("✅ {}", message);
} 