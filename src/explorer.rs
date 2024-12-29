use anyhow::Result;
use std::io::{self, Write, Stdout};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
};

pub fn read_line(prompt: &str) -> Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn show_block_details(input: &str) -> Result<()> {
    println!("\nBlock Details für {}", input);
    println!("------------------");
    // TODO: Block-Details vom Node abrufen
    Ok(())
}

pub fn show_transaction_details(txid: &str) -> Result<()> {
    println!("\nTransaktions-Details für {}", txid);
    println!("-------------------------");
    // TODO: TX-Details vom Node abrufen
    Ok(())
}

pub fn show_address_details(addr: &str) -> Result<()> {
    println!("\nAdressen-Details für {}", addr);
    println!("---------------------");
    // TODO: Adressen-Details vom Node abrufen
    Ok(())
}

pub fn handle_explorer_mode(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    // Terminal beenden
    terminal.clear()?;
    terminal.show_cursor()?;
    disable_raw_mode()?;

    // Explorer-Menü
    println!("Bitcoin Explorer");
    println!("---------------");
    println!("1) Block suchen");
    println!("2) Transaktion suchen");
    println!("3) Adresse suchen");
    println!("4) Zurück");
    
    let choice = read_line("Wähle eine Option (1-4): ")?;
    
    match choice.as_str() {
        "1" => {
            let input = read_line("Block (Hash/Höhe): ")?;
            show_block_details(&input)?;
        },
        "2" => {
            let txid = read_line("Transaktions-ID: ")?;
            show_transaction_details(&txid)?;
        },
        "3" => {
            let addr = read_line("Bitcoin-Adresse: ")?;
            show_address_details(&addr)?;
        },
        _ => return Ok(()),
    }

    // Warten auf Tastendruck
    println!("\nDrücke Enter um fortzufahren...");
    read_line("")?;

    // Terminal wiederherstellen
    enable_raw_mode()?;
    terminal.hide_cursor()?;
    Ok(())
} 