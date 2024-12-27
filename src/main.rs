mod rpc;
mod ui;

use anyhow::Result;
use crate::ui::UI;
use std::io::{self, stdout};
use crossterm::{
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
    cursor::{Hide},
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Paragraph, Block, Borders},
    layout::Alignment,
    style::{Style, Color},
    Terminal,
};

fn main() -> Result<()> {
    // Terminal sofort initialisieren
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Ladebildschirm anzeigen
    terminal.draw(|f| {
        let loading = Paragraph::new("Bitcoin Node Terminal UI wird gestartet...\nVerbinde mit Node...")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center);
        f.render_widget(loading, f.size());
    })?;

    // UI starten
    let mut ui = UI::new(terminal)?;
    ui.run()?;
    
    Ok(())
}
