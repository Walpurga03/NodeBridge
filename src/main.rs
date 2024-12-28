mod rpc;
mod ui;

use anyhow::Result;
use crate::ui::UI;
use std::io::{self, stdout, Write};
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
use crate::ui::BlockSearchMode;

fn main() -> Result<()> {
    println!("Bitcoin Node Terminal UI");
    println!("----------------------");
    println!("\nBitte wählen Sie:");
    println!("1) Aktueller Block");
    println!("2) Block nach Höhe suchen");
    println!("3) Block nach Hash suchen");
    print!("\nAuswahl (1-3): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let block_search_mode = match input.trim() {
        "2" => {
            print!("Block-Höhe eingeben: ");
            io::stdout().flush()?;
            let mut height = String::new();
            io::stdin().read_line(&mut height)?;
            BlockSearchMode::Custom(height.trim().to_string())
        },
        "3" => {
            print!("Block-Hash eingeben: ");
            io::stdout().flush()?;
            let mut hash = String::new();
            io::stdin().read_line(&mut hash)?;
            BlockSearchMode::Custom(hash.trim().to_string())
        },
        _ => BlockSearchMode::Latest
    };

    // Jetzt erst das TUI starten
    let mut ui = UI::new(block_search_mode)?;
    
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
    ui.run()?;
    
    ui.cleanup()?;
    Ok(())
}
