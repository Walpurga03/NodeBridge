mod rpc;
mod ui;

use anyhow::Result;
use crate::ui::UI;
use std::io::{self, stdout, Write};
use crossterm::{
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode},
    cursor::{Hide},
    style::{Color, Print, SetForegroundColor, ResetColor},
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Paragraph, Block, Borders},
    layout::Alignment,
    style::{Style, Color as RatatuiColor},
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
            .style(Style::default().fg(RatatuiColor::Yellow))
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
    // Terminal für bessere Darstellung löschen
    print!("\x1B[2J\x1B[1;1H");
    
    // Farbiger Header
    execute!(
        io::stdout(),
        SetForegroundColor(Color::Yellow),
        Print("\n╔════════════════════════════════════╗\n"),
        Print("║     BITCOIN NODE TERMINAL UI      ║\n"),
        Print("╚════════════════════════════════════╝\n"),
        ResetColor
    )?;

    println!("\nBitte wählen Sie eine Option aus:\n");

    // Optionen mit farbiger Hervorhebung anzeigen
    execute!(
        io::stdout(),
        SetForegroundColor(Color::Green),
        Print(" 1) "),
        ResetColor,
        Print("Block suchen (aktuell oder nach Höhe/Hash)\n")
    )?;
    
    execute!(
        io::stdout(),
        SetForegroundColor(Color::Cyan),
        Print(" 2) "),
        ResetColor,
        Print("Transaktion suchen (mit TX-ID)\n")
    )?;
    
    execute!(
        io::stdout(),
        SetForegroundColor(Color::Magenta),
        Print(" 3) "),
        ResetColor,
        Print("Adresse suchen (mit Bitcoin-Adresse)\n")
    )?;
    
    execute!(
        io::stdout(),
        SetForegroundColor(Color::Yellow),
        Print(" D) "),
        ResetColor,
        Print("Standardwerte verwenden\n")
    )?;
    
    execute!(
        io::stdout(),
        SetForegroundColor(Color::Red),
        Print(" Q) "),
        ResetColor,
        Print("Beenden\n\n")
    )?;
    
    print!("Ihre Auswahl: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();

    match input.as_str() {
        "1" => {
            // Blocksuche
            print!("\x1B[2J\x1B[1;1H"); // Clear screen
            execute!(
                io::stdout(),
                SetForegroundColor(Color::Green),
                Print("\n[ Block-Suche ]\n"),
                Print("══════════════\n\n"),
                ResetColor
            )?;
            
            print!("Block-Höhe oder Hash eingeben (oder [Enter] für aktuellen Block): ");
            io::stdout().flush()?;
            let mut block = String::new();
            io::stdin().read_line(&mut block)?;
            if block.trim().is_empty() {
                println!("\nEs wird der aktuelle Block verwendet.");
                std::thread::sleep(std::time::Duration::from_millis(1000));
                return Ok((BlockSearchMode::Latest, None, None));
            } else {
                println!("\nSuche nach Block: {}", block.trim());
                std::thread::sleep(std::time::Duration::from_millis(1000));
                return Ok((BlockSearchMode::Custom(block.trim().to_string()), None, None));
            }
        },
        "2" => {
            // Transaktionssuche
            print!("\x1B[2J\x1B[1;1H"); // Clear screen
            execute!(
                io::stdout(),
                SetForegroundColor(Color::Cyan),
                Print("\n[ Transaktions-Suche ]\n"),
                Print("═══════════════════\n\n"),
                ResetColor
            )?;
            
            println!("Standard TX-ID: {}", DEFAULT_TX_ID);
            print!("\nTransaktions-ID eingeben (oder [Enter] für Standard TX): ");
            io::stdout().flush()?;
            let mut txid = String::new();
            io::stdin().read_line(&mut txid)?;
            let txid = txid.trim();
            if txid.is_empty() {
                println!("\nEs wird die Standard-Transaktion verwendet.");
                std::thread::sleep(std::time::Duration::from_millis(1000));
                return Ok((BlockSearchMode::Latest, Some(DEFAULT_TX_ID.into()), None));
            } else {
                println!("\nSuche nach Transaktion: {}", txid);
                std::thread::sleep(std::time::Duration::from_millis(1000));
                return Ok((BlockSearchMode::Latest, Some(txid.to_string()), None));
            }
        },
        "3" => {
            // Adresssuche
            print!("\x1B[2J\x1B[1;1H"); // Clear screen
            execute!(
                io::stdout(),
                SetForegroundColor(Color::Magenta),
                Print("\n[ Adressen-Suche ]\n"),
                Print("═════════════════\n\n"),
                ResetColor
            )?;
            
            println!("Standard-Adresse: {}", DEFAULT_ADDRESS);
            print!("\nBitcoin-Adresse eingeben (oder [Enter] für Standard-Adresse): ");
            io::stdout().flush()?;
            let mut addr = String::new();
            io::stdin().read_line(&mut addr)?;
            let addr = addr.trim();
            if addr.is_empty() {
                println!("\nEs wird die Standard-Adresse verwendet.");
                std::thread::sleep(std::time::Duration::from_millis(1000));
                return Ok((BlockSearchMode::Latest, None, Some(DEFAULT_ADDRESS.into())));
            } else {
                println!("\nSuche nach Adresse: {}", addr);
                std::thread::sleep(std::time::Duration::from_millis(1000));
                return Ok((BlockSearchMode::Latest, None, Some(addr.to_string())));
            }
        },
        "d" => {
            // Standardwerte
            print!("\x1B[2J\x1B[1;1H"); // Clear screen
            execute!(
                io::stdout(),
                SetForegroundColor(Color::Yellow),
                Print("\n[ Standardwerte aktiviert ]\n"),
                Print("═════════════════════════\n\n"),
                ResetColor
            )?;
            
            println!("- Aktueller Block");
            println!("- Standard TX-ID: {}", DEFAULT_TX_ID);
            println!("- Standard-Adresse: {}", DEFAULT_ADDRESS);
            
            println!("\nProgramm wird mit Standardwerten gestartet...");
            std::thread::sleep(std::time::Duration::from_millis(1500));
            return Ok((BlockSearchMode::Latest, Some(DEFAULT_TX_ID.into()), Some(DEFAULT_ADDRESS.into())));
        },
        "q" => {
            // Beenden
            print!("\x1B[2J\x1B[1;1H"); // Clear screen
            execute!(
                io::stdout(),
                SetForegroundColor(Color::Red),
                Print("\nProgramm wird beendet...\n"),
                ResetColor
            )?;
            
            disable_raw_mode()?;
            std::thread::sleep(std::time::Duration::from_millis(500));
            std::process::exit(0);
        },
        _ => {
            // Ungültige Eingabe
            execute!(
                io::stdout(),
                SetForegroundColor(Color::Red),
                Print("\nUngültige Eingabe. Standardwerte werden verwendet.\n"),
                ResetColor
            )?;
            
            std::thread::sleep(std::time::Duration::from_millis(1500));
            return Ok((BlockSearchMode::Latest, Some(DEFAULT_TX_ID.into()), Some(DEFAULT_ADDRESS.into())));
        }
    }
}