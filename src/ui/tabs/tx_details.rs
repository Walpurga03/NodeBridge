use crate::ui::common::*;
use crate::rpc::BitcoinRPC;
use serde_json::Value;

#[derive(Clone)]
pub struct TxMode {
    pub txid: String,
}

pub fn render(mode: Option<&TxMode>, rpc_client: &Option<BitcoinRPC>) -> Paragraph<'static> {
    match (mode, rpc_client) {
        (Some(tx_mode), Some(client)) => {
            match client.get_explorer_transaction(&tx_mode.txid) {
                Ok(tx) => {
                    let mut lines = vec![
                        Line::from(vec![
                            Span::styled("Transaction Details", 
                                Style::default().fg(Color::Yellow))
                        ]),
                        Line::from(""),
                        Line::from(format!("TXID: {}", tx_mode.txid)),
                    ];

                    // Inputs
                    if let Some(inputs) = tx.get("vin").and_then(|v| v.as_array()) {
                        lines.push(Line::from(""));
                        lines.push(Line::from(vec![
                            Span::styled("Inputs:", Style::default().fg(Color::Cyan))
                        ]));
                        for input in inputs {
                            if let Some(prev_txid) = input.get("txid") {
                                lines.push(Line::from(format!(" • {}", prev_txid)));
                            }
                        }
                    }

                    // Outputs
                    if let Some(outputs) = tx.get("vout").and_then(|v| v.as_array()) {
                        lines.push(Line::from(""));
                        lines.push(Line::from(vec![
                            Span::styled("Outputs:", Style::default().fg(Color::Cyan))
                        ]));
                        for output in outputs {
                            if let Some(value) = output.get("value") {
                                if let Some(addr) = output.get("scriptPubKey")
                                    .and_then(|s| s.get("address")) {
                                    lines.push(Line::from(format!(
                                        " • {} BTC → {}", 
                                        value, addr
                                    )));
                                }
                            }
                        }
                    }

                    // Details
                    lines.extend_from_slice(&[
                        Line::from(""),
                        Line::from(vec![
                            Span::styled("Details:", Style::default().fg(Color::Cyan))
                        ]),
                        Line::from(format!(" • Größe: {} bytes", tx.get("size").unwrap_or(&Value::Null))),
                        Line::from(format!(" • Gewicht: {} WU", tx.get("weight").unwrap_or(&Value::Null))),
                        Line::from(format!(" • Zeit: {}", tx.get("time").unwrap_or(&Value::Null))),
                        Line::from(format!(" • Block: {}", tx.get("blockhash").unwrap_or(&Value::Null))),
                    ]);

                    Paragraph::new(lines)
                },
                Err(e) => Paragraph::new(format!("Fehler beim Laden der Transaktion: {}", e))
            }
        },
        _ => Paragraph::new("Keine Transaktion ausgewählt")
    }
    .block(Block::default()
        .borders(Borders::ALL)
        .title(" Transaction Details "))
} 