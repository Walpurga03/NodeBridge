use crate::ui::common::*;
use crate::rpc::BitcoinRPCInterface;

#[derive(Clone)]
pub struct AddressMode {
    pub address: String,
}

pub fn render<T: BitcoinRPCInterface>(mode: Option<&AddressMode>, rpc_client: &Option<T>) -> Paragraph<'static> {
    match (mode, rpc_client) {
        (Some(addr_mode), Some(client)) => {
            match client.get_address_details(&addr_mode.address) {
                Ok(details) => {
                    let lines = vec![
                        Line::from(vec![
                            Span::styled("📊 Adress-Details", Style::default().fg(Color::Yellow))
                        ]),
                        Line::from(""),
                        Line::from(format!("Adresse: {}", addr_mode.address)),
                        Line::from(""),
                        Line::from(vec![
                            Span::raw("Transaktionen: "),
                            Span::styled(
                                format!("{}", details.tx_count),
                                Style::default().fg(Color::Green)
                            )
                        ]),
                        Line::from(vec![
                            Span::raw("Empfangen: "),
                            Span::styled(
                                format!("{:.8} BTC", details.received),
                                Style::default().fg(Color::Green)
                            )
                        ]),
                        Line::from(vec![
                            Span::raw("Gesendet: "),
                            Span::styled(
                                format!("{:.8} BTC", details.sent),
                                Style::default().fg(Color::Red)
                            )
                        ]),
                        Line::from(vec![
                            Span::raw("Aktuelles Guthaben: "),
                            Span::styled(
                                format!("{:.8} BTC", details.balance),
                                Style::default().fg(if details.balance > 0.0 { Color::Green } else { Color::Red })
                            )
                        ]),
                        Line::from(""),
                        Line::from(vec![
                            Span::styled("📊 UTXO Statistiken", Style::default().fg(Color::Cyan))
                        ]),
                        Line::from(vec![
                            Span::raw("Empfangene Outputs: "),
                            Span::styled(
                                format!("{}", details.funded_txo_count),
                                Style::default().fg(Color::Green)
                            )
                        ]),
                        Line::from(vec![
                            Span::raw("Ausgegebene Outputs: "),
                            Span::styled(
                                format!("{}", details.spent_txo_count),
                                Style::default().fg(Color::Yellow)
                            )
                        ]),
                        Line::from(vec![
                            Span::raw("Unausgegebene Outputs: "),
                            Span::styled(
                                format!("{}", details.unspent_txo_count),
                                Style::default().fg(Color::Green)
                            )
                        ]),
                        Line::from(""),
                        Line::from(vec![
                            Span::styled("🔍 Technische Details", Style::default().fg(Color::Cyan))
                        ]),
                        Line::from(vec![
                            Span::raw("Adresstyp: "),
                            Span::styled(
                                details.address_type.clone(),
                                Style::default().fg(Color::Yellow)
                            )
                        ]),
                        Line::from(vec![
                            Span::raw("Mempool Status: "),
                            if details.has_mempool_tx {
                                Span::styled(
                                    "Unbestätigte Transaktionen vorhanden",
                                    Style::default().fg(Color::Yellow)
                                )
                            } else {
                                Span::styled(
                                    "Keine unbestätigten Transaktionen",
                                    Style::default().fg(Color::Green)
                                )
                            }
                        ]),
                    ];
                    Paragraph::new(lines)
                },
                Err(e) => Paragraph::new(format!("Fehler beim Laden der Adressdetails: {}", e))
                    .style(Style::default().fg(Color::Red))
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
            Span::styled("💰 Transaktionsübersicht", Style::default().fg(Color::Cyan))
        ]),
        Line::from(" • Transaktionen: Gesamtzahl aller Ein- und Ausgänge"),
        Line::from(" • Empfangen: Summe aller empfangenen Satoshis (1 BTC = 100.000.000 Satoshis)"),
        Line::from(" • Gesendet: Summe aller gesendeten Satoshis"),
        Line::from(" • Guthaben: Aktuell verfügbare Satoshis (Empfangen - Gesendet)"),
        Line::from(""),
        Line::from(vec![
            Span::styled("📊 UTXO Details", Style::default().fg(Color::Cyan))
        ]),
        Line::from(" • Empfangene Outputs (funded_txo_count):"),
        Line::from("   - Anzahl aller jemals empfangenen Zahlungen"),
        Line::from("   - Im Beispiel: 112 empfangene Outputs"),
        Line::from(""),
        Line::from(" • Ausgegebene Outputs (spent_txo_count):"),
        Line::from("   - Anzahl aller ausgegebenen/verbrauchten Zahlungen"),
        Line::from("   - Im Beispiel: 111 ausgegebene Outputs"),
        Line::from(""),
        Line::from(" • Unausgegebene Outputs:"),
        Line::from("   - Differenz zwischen empfangen und ausgegeben"),
        Line::from("   - Im Beispiel: 1 verfügbarer Output"),
        Line::from(""),
        Line::from(vec![
            Span::styled("🔄 Mempool Status", Style::default().fg(Color::Cyan))
        ]),
        Line::from(" • Zeigt unbestätigte Transaktionen im Mempool"),
        Line::from(" • funded_txo_sum: Neue eingehende Satoshis"),
        Line::from(" • spent_txo_sum: Neue ausgehende Satoshis"),
        Line::from(""),
        Line::from(vec![
            Span::styled("🎨 Farbkodierung", Style::default().fg(Color::Cyan))
        ]),
        Line::from(" • Grün: Verfügbare/Positive Werte (Empfangen, Guthaben)"),
        Line::from(" • Rot: Ausgegebene/Negative Werte (Gesendet)"),
        Line::from(" • Gelb: Unbestätigte/In Bearbeitung (Mempool)"),
    ])
    .block(Block::default()
        .borders(Borders::ALL)
        .title(" Hilfe: Adressdetails "))
}

fn format_timestamp(timestamp: i64) -> String {
    if timestamp == 0 {
        return "Keine Daten".to_string();
    }
    DateTime::<Utc>::from_timestamp(timestamp, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| "Ungültiges Datum".to_string())
}

#[cfg(test)]
mod tests; 