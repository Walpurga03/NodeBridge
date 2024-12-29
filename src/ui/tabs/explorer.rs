use crate::ui::common::*;
use crate::rpc::BitcoinRPC;
use serde_json::Value;

pub enum ExplorerMode {
    Transaction(String),  // Block entfernt, nur TX und Address
    Address(String),
}

pub fn render(mode: &Option<ExplorerMode>, rpc_client: &Option<BitcoinRPC>) -> Paragraph<'static> {
    match (mode, rpc_client) {
        (Some(ExplorerMode::Transaction(txid)), Some(client)) => {
            match client.get_explorer_transaction(txid) {
                Ok(tx) => {
                    Paragraph::new(vec![
                        Line::from(vec![
                            Span::styled("Transaction Details", 
                                Style::default().fg(Color::Yellow))
                        ]),
                        Line::from(""),
                        Line::from(format!("TXID: {}", tx.get("txid").unwrap_or(&Value::Null))),
                        Line::from(format!("Größe: {} bytes", tx.get("size").unwrap_or(&Value::Null))),
                        Line::from(format!("Zeit: {}", tx.get("time").unwrap_or(&Value::Null))),
                    ])
                },
                Err(e) => Paragraph::new(format!("Fehler: {}", e))
            }
        },
        (Some(ExplorerMode::Address(addr)), Some(client)) => {
            match client.get_explorer_address(addr) {
                Ok(info) => {
                    Paragraph::new(vec![
                        Line::from(vec![
                            Span::styled("Address Details", 
                                Style::default().fg(Color::Yellow))
                        ]),
                        Line::from(""),
                        Line::from(format!("Adresse: {}", addr)),
                        Line::from(format!("Typ: {}", info.get("type").unwrap_or(&Value::Null))),
                        Line::from(format!("Script: {}", info.get("scriptPubKey").unwrap_or(&Value::Null))),
                    ])
                },
                Err(e) => Paragraph::new(format!("Fehler: {}", e))
            }
        },
        _ => {
            Paragraph::new(vec![
                Line::from("Bitte wählen Sie beim Start:"),
                Line::from("3) Transaktion suchen"),
                Line::from("4) Adresse suchen"),
            ])
        }
    }
    .block(Block::default()
        .borders(Borders::ALL)
        .title(" Explorer "))
} 