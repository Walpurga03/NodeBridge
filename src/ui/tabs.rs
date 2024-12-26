// Für die verschiedenen Tab-Ansichten
use super::common::*;
use crate::ui::format_timestamp;
use crate::rpc::MempoolInfo;

pub fn create_overview(
    height: u64,
    block_hash: String,
    timestamp: i64,
    connections: u64,
    verification_progress: f64,
    mempool_size: u64,
    network: String,
) -> Paragraph<'static> {
    let content = vec![
        Line::from(vec![
            Span::styled("Netzwerk: ", Style::default().fg(Color::Cyan)),
            Span::styled(network, Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Blockhöhe: ", Style::default().fg(Color::Cyan)),
            Span::styled(height.to_string(), Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("Block Hash: ", Style::default().fg(Color::Cyan)),
            Span::styled(block_hash, Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("Zeitstempel: ", Style::default().fg(Color::Cyan)),
            Span::styled(format_timestamp(timestamp), Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Verbindungen: ", Style::default().fg(Color::Cyan)),
            Span::styled(connections.to_string(), Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("Mempool Größe: ", Style::default().fg(Color::Cyan)),
            Span::styled(format!("{} Transaktionen", mempool_size), Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("Sync Status: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{:.2}%", verification_progress * 100.0),
                Style::default().fg(if verification_progress >= 0.99 {
                    Color::Green
                } else {
                    Color::Yellow
                }),
            ),
        ]),
    ];

    Paragraph::new(content)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title(" Übersicht "))
}

pub fn create_block_details(
    height: u64,
    block_hash: String,
    timestamp: i64,
) -> Paragraph<'static> {
    let content = vec![
        Line::from(vec![
            Span::styled("Block Information", 
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Höhe: ", Style::default().fg(Color::Cyan)),
            Span::styled(height.to_string(), Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Hash: ", Style::default().fg(Color::Cyan)),
            Span::styled(block_hash, Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Zeit: ", Style::default().fg(Color::Cyan)),
            Span::styled(format_timestamp(timestamp), Style::default().fg(Color::White)),
        ]),
    ];

    Paragraph::new(content)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title(" Block Details "))
}

pub fn create_mempool(info: &MempoolInfo) -> Paragraph<'static> {
    let mut content = vec![
        Line::from(vec![
            Span::styled("Mempool Status", 
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Transaktionen: ", Style::default().fg(Color::Cyan)),
            Span::styled(info.size.to_string(), Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("Größe: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{:.2} MB", info.bytes as f64 / 1_000_000.0),
                Style::default().fg(Color::White)
            ),
        ]),
        Line::from(vec![
            Span::styled("Speichernutzung: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{:.2} MB", info.usage as f64 / 1_000_000.0),
                Style::default().fg(Color::White)
            ),
        ]),
        Line::from(vec![
            Span::styled("Gebühren (min/max): ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{:.2}/{:.2} sat/vB", info.min_fee, info.max_fee),
                Style::default().fg(Color::White)
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Gebührenverteilung (sat/vB):", 
                Style::default().fg(Color::Cyan)),
        ]),
    ];

    // Histogramm visualisieren
    if !info.fee_histogram.is_empty() {
        let total_txs: u64 = info.fee_histogram.iter()
            .map(|(_, count)| *count)
            .sum();

        // Gebührenbereiche mit Labels
        let ranges = [
            ("1-3", 0),
            ("4-6", 1),
            ("7-10", 2),
            ("11-15", 3),
            ("16-20", 4),
            ("21+", 5),
        ];

        for (label, index) in ranges {
            if let Some((fee_base, count)) = info.fee_histogram.get(index) {
                let percentage = if total_txs > 0 {
                    (*count as f64 / total_txs as f64) * 100.0
                } else {
                    0.0
                };
                
                let bar_length = (percentage * 0.4) as usize; // 40 Zeichen = 100%
                let bar = "█".repeat(bar_length);
                
                let color = match *fee_base {
                    f if f >= 21.0 => Color::Red,
                    f if f >= 16.0 => Color::LightRed,
                    f if f >= 11.0 => Color::Yellow,
                    f if f >= 7.0 => Color::Green,
                    _ => Color::LightGreen,
                };

                content.push(Line::from(vec![
                    Span::styled(
                        format!("{:>6} ", label),
                        Style::default().fg(Color::White)
                    ),
                    Span::styled(
                        format!("{:<40} ", bar),
                        Style::default().fg(color)
                    ),
                    Span::styled(
                        format!("{:>5.1}%", percentage),
                        Style::default().fg(Color::Gray)
                    ),
                    Span::styled(
                        format!(" ({} Transaktionen)", count),
                        Style::default().fg(Color::DarkGray)
                    ),
                ]));
            }
        }

        // Zusammenfassung
        content.push(Line::from(""));
        content.push(Line::from(vec![
            Span::styled("Gesamt: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{} Transaktionen", total_txs),
                Style::default().fg(Color::White)
            ),
        ]));
        content.push(Line::from(vec![
            Span::styled("Gebührenbereich: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{:.1} - {:.1} sat/vB", info.min_fee, info.max_fee),
                Style::default().fg(Color::White)
            ),
        ]));
    } else {
        content.push(Line::from(vec![
            Span::styled(
                "Keine Transaktionen im Mempool",
                Style::default().fg(Color::Gray)
            ),
        ]));
    }

    Paragraph::new(content)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title(" Mempool "))
}

pub fn create_network(
    connections: u64,
    network: String,
    verification_progress: f64,
) -> Paragraph<'static> {
    let content = vec![
        Line::from(vec![
            Span::styled("Netzwerk Status", 
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Netzwerk: ", Style::default().fg(Color::Cyan)),
            Span::styled(network, Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Verbindungen: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                connections.to_string(),
                Style::default().fg(if connections < 3 {
                    Color::Red
                } else if connections < 8 {
                    Color::Yellow
                } else {
                    Color::Green
                }),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Synchronisation: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{:.2}%", verification_progress * 100.0),
                Style::default().fg(if verification_progress >= 0.99 {
                    Color::Green
                } else {
                    Color::Yellow
                }),
            ),
        ]),
    ];

    Paragraph::new(content)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title(" Netzwerk Status "))
} 