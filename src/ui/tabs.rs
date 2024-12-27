// Für die verschiedenen Tab-Ansichten
use super::common::*;
use crate::ui::format_timestamp;
use crate::rpc::MempoolInfo;
use crate::rpc::PeerInfo;
use ratatui::widgets::{Table, Row};

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

    // Gebühren-Histogramm
    if !info.fee_histogram.is_empty() {
        let fee_ranges = [
            ("1-3", 0),
            ("4-6", 1),
            ("7-10", 2),
            ("11-15", 3),
            ("16-20", 4),
            ("21+", 5),
        ];

        let total_txs: u64 = info.fee_histogram.iter()
            .map(|(_, count)| *count)
            .sum();

        for (label, index) in fee_ranges {
            if let Some((fee_base, count)) = info.fee_histogram.get(index) {
                let percentage = if total_txs > 0 {
                    (*count as f64 / total_txs as f64) * 100.0
                } else {
                    0.0
                };
                
                let bar_length = (percentage * 0.4) as usize;
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
                        format!("{:>5.1}% ({} TX)", percentage, count),
                        Style::default().fg(Color::Gray)
                    ),
                ]));
            }
        }
    }

    // 2. Größenverteilung
    content.push(Line::from(""));
    content.push(Line::from(vec![
        Span::styled("Größenverteilung:", 
            Style::default().fg(Color::Cyan)),
    ]));

    let size_labels = [
        ("< 500 vB", 0),
        ("< 1000 vB", 1),
        ("< 2000 vB", 2),
        ("< 5000 vB", 3),
        ("> 5000 vB", 4),
    ];

    let total_txs: u64 = info.size_distribution.iter()
        .map(|(_, count)| *count)
        .sum();

    for (label, index) in size_labels {
        if let Some((_, count)) = info.size_distribution.get(index) {
            let percentage = if total_txs > 0 {
                (*count as f64 / total_txs as f64) * 100.0
            } else {
                0.0
            };
            
            let bar_length = (percentage * 0.4) as usize;
            let bar = "█".repeat(bar_length);
            
            content.push(Line::from(vec![
                Span::styled(
                    format!("{:>10} ", label),
                    Style::default().fg(Color::White)
                ),
                Span::styled(
                    format!("{:<40} ", bar),
                    Style::default().fg(Color::Blue)
                ),
                Span::styled(
                    format!("{:>5.1}% ({} TX)", percentage, count),
                    Style::default().fg(Color::Gray)
                ),
            ]));
        }
    }

    // 3. Altersverteilung
    content.push(Line::from(""));
    content.push(Line::from(vec![
        Span::styled("Altersverteilung:", 
            Style::default().fg(Color::Cyan)),
    ]));

    let age_labels = [
        ("< 10 min", 0),
        ("< 30 min", 1),
        ("< 1h", 2),
        ("< 3h", 3),
        ("< 6h", 4),
        ("> 6h", 5),
    ];

    for (label, index) in age_labels {
        if let Some((_, count)) = info.age_distribution.get(index) {
            let percentage = if total_txs > 0 {
                (*count as f64 / total_txs as f64) * 100.0
            } else {
                0.0
            };
            
            let bar_length = (percentage * 0.4) as usize;
            let bar = "█".repeat(bar_length);
            
            content.push(Line::from(vec![
                Span::styled(
                    format!("{:>10} ", label),
                    Style::default().fg(Color::White)
                ),
                Span::styled(
                    format!("{:<40} ", bar),
                    Style::default().fg(Color::Magenta)
                ),
                Span::styled(
                    format!("{:>5.1}% ({} TX)", percentage, count),
                    Style::default().fg(Color::Gray)
                ),
            ]));
        }
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
    peers: &[PeerInfo],
) -> Paragraph<'_> {
    // Onion-Adressen zählen
    let onion_count = peers.iter()
        .filter(|p| p.addr.contains(".onion"))
        .count();

    // Beste und schlechteste Latenz finden
    let (best_peer, worst_peer) = peers.iter()
        .fold((None, None), |(min, max), peer| {
            match (min, max) {
                (None, None) => (Some(peer), Some(peer)),
                (Some(min_peer), Some(max_peer)) => (
                    if peer.latency < min_peer.latency { Some(peer) } else { Some(min_peer) },
                    if peer.latency > max_peer.latency { Some(peer) } else { Some(max_peer) }
                ),
                _ => (Some(peer), Some(peer))
            }
        });

    let mut content = vec![
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
                format!("{} ({} davon Onion)", connections, onion_count),
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

    // Peer-Latenz-Details
    if let (Some(best), Some(worst)) = (best_peer, worst_peer) {
        content.extend(vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("Beste Latenz: ", Style::default().fg(Color::Cyan)),
                Span::styled(&best.addr, Style::default().fg(Color::White)),
                Span::raw(" - "),
                Span::styled(
                    format!("{:.0}ms", best.latency * 1000.0),
                    Style::default().fg(Color::Green),
                ),
            ]),
            Line::from(vec![
                Span::styled("Schlechteste Latenz: ", Style::default().fg(Color::Cyan)),
                Span::styled(&worst.addr, Style::default().fg(Color::White)),
                Span::raw(" - "),
                Span::styled(
                    format!("{:.0}ms", worst.latency * 1000.0),
                    Style::default().fg(Color::Red),
                ),
            ]),
        ]);
    }

    Paragraph::new(content)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title(" Netzwerk Status "))
}

pub fn create_network_details(peers: &[PeerInfo]) -> Paragraph<'_> {
    let mut content = vec![
        Line::from(vec![
            Span::styled("Peer Details", 
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
    ];

    for peer in peers {
        content.extend(vec![
            Line::from(vec![
                Span::styled("Adresse: ", Style::default().fg(Color::Cyan)),
                Span::styled(&peer.addr, Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled("Version: ", Style::default().fg(Color::Cyan)),
                Span::styled(&peer.subver, Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled("Latenz: ", Style::default().fg(Color::Cyan)),
                Span::styled(
                    format!("{:.2}ms", peer.latency * 1000.0),
                    Style::default().fg(if peer.latency < 0.1 {
                        Color::Green
                    } else if peer.latency < 0.5 {
                        Color::Yellow
                    } else {
                        Color::Red
                    }),
                ),
            ]),
            Line::from(""),
        ]);
    }

    Paragraph::new(content)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title(" Netzwerk Details "))
}

pub fn create_peer_list(peers: &[PeerInfo]) -> Table<'_> {
    let header = Row::new(vec![
        "Typ",
        "Adresse",
        "Latenz",
    ]).style(Style::default().fg(Color::Yellow));

    // Onion-Adressen zählen
    let onion_count = peers.iter()
        .filter(|p| p.addr.contains(".onion"))
        .count();

    // Beste und schlechteste Latenz finden
    let (best_latency, worst_latency) = peers.iter()
        .fold((f64::MAX, 0.0_f64), |(min, max): (f64, f64), peer| {
            (min.min(peer.latency), max.max(peer.latency))
        });

    // Peers nach Latenz sortieren und in Rows umwandeln
    let mut peers_sorted = peers.to_vec();
    peers_sorted.sort_by(|a, b| a.latency.partial_cmp(&b.latency).unwrap());

    let mut rows: Vec<Row> = peers_sorted.iter().map(|p| {
        let latency_style = if (p.latency - best_latency).abs() < 0.001 {
            Style::default().fg(Color::Green)
        } else if (p.latency - worst_latency).abs() < 0.001 {
            Style::default().fg(Color::Red)
        } else {
            Style::default().fg(Color::White)
        };

        Row::new(vec![
            if p.addr.contains(".onion") { "Onion" } else { "Clear" }.to_string(),
            p.addr.clone(),
            format!("{:.0}ms", p.latency * 1000.0),
        ]).style(latency_style)
    }).collect();

    // Zusammenfassung am Anfang
    let summary = Row::new(vec![
        "ZUSAMMENFASSUNG".to_string(),
        format!("Onion: {}/{}", onion_count, peers.len()),
        format!("Latenz: {:.0}-{:.0}ms", best_latency * 1000.0, worst_latency * 1000.0),
    ]).style(Style::default().fg(Color::Cyan));

    rows.insert(0, summary);

    Table::new(rows)
        .header(header)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" Peer Liste "))
        .widths(&[
            Constraint::Percentage(15),  // Typ
            Constraint::Percentage(65),  // Adresse
            Constraint::Percentage(20),  // Latenz
        ])
        .column_spacing(1)
        .highlight_style(Style::default().fg(Color::Yellow))
}

pub fn create_mining(/* Parameter hier */) -> Paragraph<'static> {
    let content = vec![
        Line::from(vec![
            Span::styled("Mining Status", 
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Coming Soon!", Style::default().fg(Color::Gray)),
        ]),
    ];

    Paragraph::new(content)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title(" Mining "))
}

pub fn create_security(/* Parameter hier */) -> Paragraph<'static> {
    let content = vec![
        Line::from(vec![
            Span::styled("Sicherheitsstatus", 
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Coming Soon!", Style::default().fg(Color::Gray)),
        ]),
    ];

    Paragraph::new(content)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title(" Sicherheit "))
}

pub fn create_explorer() -> Paragraph<'static> {
    let content = vec![
        Line::from(vec![
            Span::styled("Blockchain Explorer", 
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from("Suche nach:"),
        Line::from(vec![
            Span::styled("[B]", Style::default().fg(Color::Yellow)),
            Span::raw(" Block (Höhe/Hash)"),
        ]),
        Line::from(vec![
            Span::styled("[T]", Style::default().fg(Color::Yellow)),
            Span::raw(" Transaktion (TXID)"),
        ]),
        Line::from(vec![
            Span::styled("[A]", Style::default().fg(Color::Yellow)),
            Span::raw(" Adresse"),
        ]),
        Line::from(""),
        Line::from("Eingabe: _____________"),  // Platzhalter für Suchfeld
        Line::from(""),
        Line::from("Ergebnisse werden hier angezeigt..."),
    ];

    Paragraph::new(content)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title(" Explorer "))
} 