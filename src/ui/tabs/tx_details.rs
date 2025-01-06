use super::super::common::*;
use crate::rpc::BitcoinRPC;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct TxMode {
    pub txid: String,
}

impl TxMode {
    pub fn new(txid: String) -> Self {
        Self {
            txid,
        }
    }
}

pub fn render(mode: Option<&TxMode>, rpc_client: &Option<BitcoinRPC>) -> Paragraph<'static> {
    match (mode, rpc_client) {
        (Some(tx_mode), Some(client)) => {
            match client.get_raw_transaction(&tx_mode.txid) {
                Ok(tx) => {
                    let mut lines = vec![
                        Line::from(vec![
                            Span::styled("Transaction Details", 
                                Style::default().fg(Color::Yellow))
                        ]),
                        Line::from(""),
                    ];

                    // TXID
                    lines.push(Line::from(vec![
                        Span::raw("TXID: "),
                        Span::styled(
                            tx_mode.txid.clone(),
                            Style::default().fg(Color::White)
                        )
                    ]));

                    // Prüfen ob es eine Coinbase TX ist
                    let is_coinbase = tx.vin.len() == 1 && 
                        tx.vin[0].get("txid")
                           .and_then(|v| v.as_str())
                           .map(|s| s == "0000000000000000000000000000000000000000000000000000000000000000")
                           .unwrap_or(false);

                    if is_coinbase {
                        lines.push(Line::from(""));
                        lines.push(Line::from(vec![
                            Span::styled("🌟 Coinbase Transaktion (Block Belohnung + Gebühren)", 
                                Style::default().fg(Color::Yellow))
                        ]));
                    }

                    // Inputs
                    if !tx.vin.is_empty() {
                        lines.push(Line::from(""));
                        lines.push(Line::from(vec![
                            Span::styled("Inputs:", Style::default().fg(Color::Cyan))
                        ]));
                        if is_coinbase {
                            lines.push(Line::from(" • Neue Bitcoins (Block Belohnung)"));
                        } else {
                            for input in tx.vin.iter() {
                                let value = input.get("value").cloned();
                                let addr = input.get("address").cloned();
                                let txid = input.get("txid").cloned();
                                
                                if let (Some(value), Some(addr)) = (value, addr) {
                                    lines.push(Line::from(format!(
                                        " • {} BTC", value
                                    )));
                                    lines.push(Line::from(vec![
                                        Span::raw("   TX: "),
                                        Span::styled(
                                            txid.and_then(|v| v.as_str().map(|s| s.to_string()))
                                                .unwrap_or_else(|| "Unbekannt".to_string()),
                                            Style::default()
                                        )
                                    ]));
                                    lines.push(Line::from(vec![
                                        Span::raw("   Von: "),
                                        Span::styled(
                                            addr.as_str().map(|s| s.to_string())
                                                .unwrap_or_else(|| "Unbekannt".to_string()),
                                            Style::default()
                                        )
                                    ]));
                                }
                            }
                        }
                    }

                    // Outputs
                    if !tx.vout.is_empty() {
                        lines.push(Line::from(""));
                        lines.push(Line::from(vec![
                            Span::styled("Outputs:", Style::default().fg(Color::Cyan))
                        ]));
                        for (index, output) in tx.vout.iter().enumerate() {
                            let value = output.get("value").cloned();
                            let script_pub_key = output.get("scriptPubKey").cloned();
                            
                            if let Some(value) = value {
                                lines.push(Line::from(format!(
                                    " • {} BTC", value
                                 )));
                                lines.push(Line::from(vec![
                                    Span::raw("   TX: "),
                                    Span::styled(
                                        format!("{}:{}", tx_mode.txid, index),
                                        Style::default()
                                    )
                                ]));
                                if let Some(script_pub_key) = script_pub_key {
                                    if let Some(addr) = script_pub_key.get("address") {
                                        lines.push(Line::from(vec![
                                            Span::raw("   An: "),
                                            Span::styled(
                                                addr.as_str().map(|s| s.to_string())
                                                    .unwrap_or_else(|| "Unbekannt".to_string()),
                                                Style::default()
                                            )
                                        ]));
                                    }
                                }
                            }
                        }
                    }

                    // Zeit formatieren
                    let dt = if let Some(blocktime) = tx.blocktime {
                        DateTime::<Utc>::from_timestamp(blocktime as i64, 0)
                            .unwrap_or_default()
                            .format("%Y-%m-%d %H:%M:%S UTC").to_string()
                    } else {
                        "Noch nicht bestätigt".to_string()
                    };

                    // Details
                    lines.extend_from_slice(&[
                        Line::from(""),
                        Line::from(vec![
                            Span::styled("Details:", Style::default().fg(Color::Cyan))
                        ]),
                        Line::from(vec![
                            Span::styled("Größe: ", Style::default().fg(Color::Cyan)),
                            Span::styled(
                                format!("{} bytes", tx.size),
                                Style::default().fg(Color::White)
                            )
                        ]),
                        Line::from(vec![
                            Span::styled("Virtuelle Größe: ", Style::default().fg(Color::Cyan)),
                            Span::styled(
                                format!("{} vbytes", tx.size),
                                Style::default().fg(Color::White)
                            )
                        ]),
                        Line::from(vec![
                            Span::styled("Gewicht: ", Style::default().fg(Color::Cyan)),
                            Span::styled(
                                format!("{} WU", tx.weight),
                                Style::default().fg(Color::White)
                            )
                        ]),
                        Line::from(vec![
                            Span::styled("Zeit: ", Style::default().fg(Color::Cyan)),
                            Span::styled(dt, Style::default().fg(Color::White)),
                        ]),
                        Line::from(vec![
                            Span::styled("Block: ", Style::default().fg(Color::Cyan)),
                            Span::styled(
                                tx.blockhash.unwrap_or_else(|| "Noch nicht bestätigt".to_string()),
                                Style::default().fg(Color::White)
                            ),
                        ]),
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

#[allow(dead_code)]
pub fn some_unused_function() {
    // Funktionaler Code hier
} 