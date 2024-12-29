use crate::ui::common::*;
use crate::rpc::BitcoinRPC;
use serde_json::Value;
use std::fs::OpenOptions;
use std::io::Write;
use bitcoincore_rpc::Error;

pub trait ExplorerAddress: Clone {
    fn get_explorer_address(&self, address: &str) -> Result<Value, Error>;
}

impl ExplorerAddress for BitcoinRPC {
    fn get_explorer_address(&self, address: &str) -> Result<Value, Error> {
        BitcoinRPC::get_explorer_address(self, address)
    }
}

#[derive(Clone)]
pub struct AddressMode {
    pub address: String,
}

pub fn render<T: ExplorerAddress + Clone>(mode: Option<&AddressMode>, rpc_client: &Option<T>) -> Paragraph<'static> {
    // Debug Log File leeren
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)  // Diese Option leert die Datei
        .open("debug.log") 
    {
        let _ = writeln!(file, "=== Debug Log Start ===");
    }

    // Logger Funktion
    #[allow(dead_code)]
    fn log_debug(msg: &str) {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open("debug.log") 
        {
            let _ = writeln!(file, "{}", msg);
        }
    }

    match (mode, rpc_client) {
        (Some(addr_mode), Some(client)) => {
            match client.get_explorer_address(&addr_mode.address) {
                Ok(info) => {
                    // Alle Werte aus info extrahieren
                    let mut lines = Vec::new();
                    
                    // Grundlegende Werte extrahieren
                    let balance = info.get("balance").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let total_received = info.get("total_received").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let utxo_count = info.get("utxo_details")
                        .and_then(|u| u.get("count"))
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0);
                    let unconfirmed_utxos = info.get("unconfirmed_utxos")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0);
                    let tx_count = info.get("tx_count").and_then(|v| v.as_u64()).unwrap_or(0);

                    // Transaktionen in eigene Struktur extrahieren
                    let transactions: Vec<Value> = info.get("recent_transactions")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.to_vec())
                        .unwrap_or_default();

                    // Header und Adresstyp
                    lines.extend_from_slice(&[
                        Line::from(vec![
                            Span::styled("Address Details", Style::default().fg(Color::Yellow))
                        ]),
                        Line::from(""),
                        Line::from(format!("Adresse: {}", addr_mode.address)),
                    ]);

                    // Adresstyp und Status
                    let addr_type = if addr_mode.address.starts_with("bc1p") {
                        "Taproot (P2TR)"
                    } else if addr_mode.address.starts_with("bc1") {
                        "Native SegWit (bech32)"
                    } else if addr_mode.address.starts_with("3") {
                        "Nested SegWit (P2SH)"
                    } else if addr_mode.address.starts_with("1") {
                        "Legacy (P2PKH)"
                    } else if addr_mode.address.len() == 130 {
                        "Public Key (Hex)"
                    } else {
                        "Unbekanntes Format"
                    };

                    // Adresstyp & Status Block
                    lines.extend_from_slice(&[
                        Line::from(""),
                        Line::from(vec![
                            Span::styled("🔑 Adresstyp & Status:", Style::default().fg(Color::Cyan))
                        ]),
                        Line::from(format!(" • Typ: {}", addr_type)),
                    ]);

                    // Finanzieller Status Block
                    lines.extend_from_slice(&[
                        Line::from(""),
                        Line::from(vec![
                            Span::styled("💰 Finanzieller Status:", Style::default().fg(Color::Cyan))
                        ]),
                        Line::from(vec![
                            Span::raw(" • Bestätigter Guthaben: "),
                            Span::styled(
                                format!("{:.8} BTC", balance),
                                Style::default().fg(Color::Green)
                            ),
                        ]),
                        Line::from(format!(" • Insgesamt empfangen: {:.8} BTC", total_received)),
                    ]);

                    // UTXO Details Block
                    lines.extend_from_slice(&[
                        Line::from(""),
                        Line::from(vec![
                            Span::styled("🔗 UTXO Details:", Style::default().fg(Color::Cyan))
                        ]),
                        Line::from(format!(" • Bestätigte UTXOs: {}", utxo_count)),
                        Line::from(format!(" • Ausstehende UTXOs: {}", unconfirmed_utxos)),
                    ]);

                    // Transaktionshistorie Block
                    if !transactions.is_empty() {
                        lines.extend_from_slice(&[
                            Line::from(""),
                            Line::from(vec![
                                Span::styled("📊 Transaktionshistorie:", Style::default().fg(Color::Cyan))
                            ]),
                            Line::from(format!(" • Anzahl Transaktionen: {}", tx_count)),
                            Line::from(""),
                        ]);

                        for tx in transactions {
                            // Alle Werte aus der Transaktion extrahieren
                            let txid = tx.get("txid").and_then(|v| v.as_str()).unwrap_or("Unbekannt").to_string();
                            let amount = tx.get("amount").and_then(|v| v.as_f64()).unwrap_or(0.0);
                            let time = tx.get("time").and_then(|v| v.as_i64()).unwrap_or(0);
                            let datetime = chrono::DateTime::from_timestamp(time, 0)
                                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                                .unwrap_or_else(|| "Unbekannt".to_string());

                            // Details extrahieren und klonen
                            let details: Option<Value> = tx.get("details").map(|d| d.clone());
                            
                            // Transaktion Header
                            lines.push(Line::from(vec![
                                Span::styled("━".repeat(50), Style::default().fg(Color::DarkGray))
                            ]));

                            // TXID mit Zeitstempel
                            lines.extend_from_slice(&[
                                Line::from(vec![
                                    Span::styled("TXID: ", Style::default().fg(Color::Yellow)),
                                    Span::raw(txid),
                                ]),
                                Line::from(vec![
                                    Span::styled("Zeit: ", Style::default().fg(Color::Yellow)),
                                    Span::raw(datetime),
                                ]),
                            ]);

                            // Betrag mit Farbe
                            lines.push(Line::from(vec![
                                Span::styled("Betrag: ", Style::default().fg(Color::Yellow)),
                                Span::styled(
                                    format!("{:+.8} BTC", amount),
                                    Style::default().fg(if amount > 0.0 { Color::Green } else { Color::Red })
                                ),
                            ]));

                            // Details anzeigen
                            if let Some(details) = details {
                                if let Some(from) = details.get("from").and_then(|v| v.as_str()) {
                                    lines.push(Line::from(vec![
                                        Span::styled("Von: ", Style::default().fg(Color::Yellow)),
                                        Span::raw(from.to_string()),
                                    ]));
                                }

                                // Outputs anzeigen
                                if let Some(outputs) = details.get("outputs").and_then(|v| v.as_array()) {
                                    lines.push(Line::from(vec![
                                        Span::styled("Outputs:", Style::default().fg(Color::Yellow))
                                    ]));
                                    
                                    for output in outputs {
                                        if let (Some(addr), Some(amt)) = (
                                            output.get("address").and_then(|v| v.as_str()),
                                            output.get("amount").and_then(|v| v.as_f64())
                                        ) {
                                            lines.push(Line::from(vec![
                                                Span::raw("  → "),
                                                Span::styled(
                                                    format!("{:.8} BTC", amt),
                                                    Style::default().fg(Color::Green)
                                                ),
                                                Span::raw(" an "),
                                                Span::raw(addr.to_string()),
                                            ]));
                                        }
                                    }
                                }

                                // Gebühren anzeigen
                                if let Some(fee) = details.get("fee").and_then(|v| v.as_f64()) {
                                    lines.push(Line::from(vec![
                                        Span::styled("Gebühr: ", Style::default().fg(Color::Yellow)),
                                        Span::raw(format!("{:.8} BTC", fee)),
                                    ]));
                                }
                            }
                        }
                    }

                    Paragraph::new(lines)
                },
                Err(e) => {
                    let error_msg = match e.to_string().as_str() {
                        s if s.contains("Invalid address") => 
                            "Ungültige Adresse: Bitte überprüfen Sie das Format",
                        s if s.contains("not found") => 
                            "Adresse nicht gefunden: Möglicherweise noch keine Transaktionen",
                        _ => "Fehler beim Laden der Adresse"
                    };
                    
                    Paragraph::new(vec![
                        Line::from(vec![
                            Span::styled("⚠️ Fehler", Style::default().fg(Color::Red))
                        ]),
                        Line::from(""),
                        Line::from(error_msg),
                        Line::from(""),
                        Line::from("Unterstützte Formate:"),
                        Line::from(" • Legacy: 1..."),
                        Line::from(" • SegWit: 3..."),
                        Line::from(" • Native SegWit: bc1..."),
                        Line::from(" • Public Key: 130 Zeichen (hex)"),
                    ])
                }
            }
        },
        _ => Paragraph::new("Keine Adresse ausgewählt")
    }
    .block(Block::default()
        .borders(Borders::ALL)
        .title(" Address Details "))
}

/// Gibt die Hilfe-Dokumentation für die Adressdetails zurück
#[allow(dead_code)]
pub fn render_help() -> Paragraph<'static> {
    Paragraph::new(vec![
        Line::from(vec![
            Span::styled("📚 Adressdetails Hilfe", Style::default().fg(Color::Yellow))
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Übersicht", Style::default().fg(Color::Cyan))
        ]),
        Line::from("Die Adressdetails zeigen umfassende Informationen zu einer Bitcoin-Adresse:"),
        Line::from(""),
        Line::from(vec![
            Span::styled("🔑 Adresstyp & Status", Style::default().fg(Color::Cyan))
        ]),
        Line::from(" • Zeigt den Typ der Adresse (Legacy, SegWit, Taproot etc.)"),
        Line::from(" • Unterstützte Formate:"),
        Line::from("   - Legacy (1...)"),
        Line::from("   - SegWit (3...)"), 
        Line::from("   - Native SegWit (bc1...)"),
        Line::from("   - Taproot (bc1p...)"),
        Line::from(""),
        Line::from(vec![
            Span::styled("💰 Finanzieller Status", Style::default().fg(Color::Cyan))
        ]),
        Line::from(" • Bestätigter Guthaben: Aktuell verfügbarer Betrag"),
        Line::from(" • Insgesamt empfangen: Summe aller eingegangenen Transaktionen"),
        Line::from(""),
        Line::from(vec![
            Span::styled("🔗 UTXO Details", Style::default().fg(Color::Cyan))
        ]),
        Line::from(" • Bestätigte UTXOs: Anzahl der verfügbaren Outputs"),
        Line::from(" • Ausstehende UTXOs: Noch unbestätigte Outputs"),
        Line::from(""),
        Line::from(vec![
            Span::styled("📊 Transaktionshistorie", Style::default().fg(Color::Cyan))
        ]),
        Line::from(" • Listet die letzten Transaktionen der Adresse"),
        Line::from(" • Pro Transaktion werden angezeigt:"),
        Line::from("   - TXID: Eindeutige Transaktions-ID"),
        Line::from("   - Zeit: Zeitpunkt der Transaktion"),
        Line::from("   - Betrag: Ein- oder ausgehender Betrag"),
        Line::from("   - Von: Absenderadresse"),
        Line::from("   - Outputs: Liste der Empfänger und Beträge"),
        Line::from("   - Gebühr: Transaktionsgebühr (wenn verfügbar)"),
        Line::from(""),
        Line::from(vec![
            Span::styled("🎨 Farbkodierung", Style::default().fg(Color::Cyan))
        ]),
        Line::from(" • Grün: Eingehende Beträge"),
        Line::from(" • Rot: Ausgehende Beträge"),
        Line::from(" • Gelb: Wichtige Überschriften und IDs"),
    ])
    .block(Block::default()
        .borders(Borders::ALL)
        .title(" Hilfe: Adressdetails "))
}

#[cfg(test)]
mod tests; 