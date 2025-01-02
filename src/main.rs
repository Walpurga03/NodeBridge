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
use log::{info};
use env_logger;

const DEFAULT_TX_ID: &str = "b8ba9eb64978b378e7b03e25d14062c10ea844a284d87552c808ab4f4365c958";
const DEFAULT_ADDRESS: &str = "bc1p38hzyl8p5yyqnzgkcxttr6ac0wc0ae8gpv7rld79df88qkrva38s78e8wd";

fn main() -> Result<()> {
    // Initialisiere das Logging-System
    env_logger::init();
    info!("Bitcoin Node Terminal UI wird gestartet.");

    // Separate Variablen für Block und Explorer
    let (block_mode, tx_id, addr) = get_user_selection()?;
    
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
    info!("Ladebildschirm angezeigt.");

    // UI starten
    ui.run()?;
    info!("UI läuft.");
    
    ui.cleanup()?;
    info!("UI bereinigt.");
    Ok(())
}

fn get_user_selection() -> Result<(BlockSearchMode, Option<String>, Option<String>)> {
    println!("\nBitte wählen Sie eine Option aus:");
    println!("1) Block suchen");
    println!("2) Transaktion suchen");
    println!("3) Adresse suchen");
    println!();
    println!("Geben Sie die Nummer der gewünschten Option ein und drücken Sie [Enter].");
    println!("Drücken Sie einfach [Enter], um alle Standardwerte zu verwenden.");
    print!("\nIhre Auswahl: ");
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
                 Some(DEFAULT_TX_ID.into()), 
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
                 Some(DEFAULT_ADDRESS.into()))
            } else {
                (BlockSearchMode::Latest, None, Some(addr.to_string()))
            }
        },
        "" => {
            // Bei [Enter] alle Standardwerte verwenden
            (BlockSearchMode::Latest, 
             Some(DEFAULT_TX_ID.into()),
             Some(DEFAULT_ADDRESS.into()))
        },
        _ => {
            println!("Ungültige Auswahl. Bitte geben Sie eine Zahl zwischen 1 und 3 ein oder drücken Sie [Enter].");
            // Optional: Wiederholen der Auswahlaufforderung oder Standardwerte verwenden
            (BlockSearchMode::Latest, Some(DEFAULT_TX_ID.into()), Some(DEFAULT_ADDRESS.into()))
        }
    };

    Ok((block_mode, tx_id, addr))
}
