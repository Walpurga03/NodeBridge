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
    println!("1) Block suchen (Standard: Aktueller Block)");
    println!("2) Transaktion suchen (Standard: Standard TX)");
    println!("3) Adresse suchen (Standard: Standard-Adresse)");
    print!("\nAuswahl (1-3) oder [Enter] für alle Standardwerte: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Separate Variablen für Block und Explorer
    let (block_mode, tx_id, addr) = match input.trim() {
        "1" => {
            print!("Block-Höhe oder Hash eingeben (oder [Enter] für aktuellen Block): ");
            io::stdout().flush()?;
            let mut block = String::new();
            io::stdin().read_line(&mut block)?;
            if block.trim().is_empty() {
                (BlockSearchMode::Latest, None, None)
            } else {
                (BlockSearchMode::Custom(block.trim().to_string()), None, None)
            }
        },
        "2" => {
            print!("Transaktions-ID eingeben (oder [Enter] für Standard TX): ");
            io::stdout().flush()?;
            let mut txid = String::new();
            io::stdin().read_line(&mut txid)?;
            let txid = txid.trim();
            if txid.is_empty() {
                (BlockSearchMode::Latest, 
                 Some("b8ba9eb64978b378e7b03e25d14062c10ea844a284d87552c808ab4f4365c958".into()), 
                 None)
            } else {
                (BlockSearchMode::Latest, Some(txid.to_string()), None)
            }
        },
        "3" => {
            print!("Bitcoin-Adresse eingeben (oder [Enter] für Standard-Adresse): ");
            io::stdout().flush()?;
            let mut addr = String::new();
            io::stdin().read_line(&mut addr)?;
            let addr = addr.trim();
            if addr.is_empty() {
                (BlockSearchMode::Latest, None, 
                 Some("bc1p38hzyl8p5yyqnzgkcxttr6ac0wc0ae8gpv7rld79df88qkrva38s78e8wd".into()))
            } else {
                (BlockSearchMode::Latest, None, Some(addr.to_string()))
            }
        },
        "" => {
            // Bei [Enter] alle Standardwerte verwenden
            (BlockSearchMode::Latest, 
             Some("b8ba9eb64978b378e7b03e25d14062c10ea844a284d87552c808ab4f4365c958".into()),
             Some("bc1p38hzyl8p5yyqnzgkcxttr6ac0wc0ae8gpv7rld79df88qkrva38s78e8wd".into()))
        },
        _ => (BlockSearchMode::Latest, None, None)
    };

    // UI mit beiden Modi starten
    let mut ui = UI::new(block_mode, tx_id, addr)?;
    
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
