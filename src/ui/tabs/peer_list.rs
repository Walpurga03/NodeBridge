use crate::ui::common::*;
use crate::rpc::PeerInfo;
use ratatui::widgets::{Table, Row};
use chrono::{DateTime, Utc};

pub fn render(peers: &[PeerInfo]) -> Table<'static> {
    // Spaltenüberschriften
    let header = Row::new(vec![
        "Adresse",
        "Version",
        "Latenz",
        "Verbunden",
        "Senden/Empfangen",
        "Typ",
    ])
    .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

    // Peer-Daten sortiert nach Latenz
    let mut sorted_peers = peers.to_vec();
    sorted_peers.sort_by(|a, b| a.latency.partial_cmp(&b.latency).unwrap_or(std::cmp::Ordering::Equal));

    let rows: Vec<Row> = sorted_peers.iter().map(|peer| {
        // Zeit seit Verbindungsaufbau berechnen
        let connected_since = DateTime::<Utc>::from_timestamp(peer.connected_time as i64, 0)
            .unwrap_or_default();
        let duration = Utc::now().signed_duration_since(connected_since);
        let connected_time = if duration.num_days() > 0 {
            format!("{}d", duration.num_days())
        } else if duration.num_hours() > 0 {
            format!("{}h", duration.num_hours())
        } else {
            format!("{}m", duration.num_minutes())
        };

        // Datenübertragung formatieren
        let transfer = format!("{:.1}MB/{:.1}MB",
            peer.bytes_sent as f64 / 1_000_000.0,
            peer.bytes_recv as f64 / 1_000_000.0
        );

        Row::new(vec![
            // Adresse (mit Tor-Markierung)
            if peer.addr.contains(".onion") {
                format!("🧅 {}", peer.addr)
            } else {
                peer.addr.clone()
            },
            // Version
            peer.subver.clone(),
            // Latenz mit Farbkodierung
            format!("{:.0}ms", peer.latency),
            // Verbindungszeit
            connected_time,
            // Datenübertragung
            transfer,
            // Verbindungstyp
            if peer.inbound { "Eingehend" } else { "Ausgehend" }.to_string(),
        ])
        .style(Style::default().fg(
            if peer.latency < 100.0 {
                Color::Green
            } else if peer.latency < 500.0 {
                Color::Yellow
            } else {
                Color::Red
            }
        ))
    }).collect();

    Table::new(rows)
        .header(header)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" Peer Liste "))
        .widths(&[
            Constraint::Percentage(30), // Adresse
            Constraint::Percentage(20), // Version
            Constraint::Percentage(10), // Latenz
            Constraint::Percentage(10), // Verbunden
            Constraint::Percentage(15), // Transfer
            Constraint::Percentage(15), // Typ
        ])
        .column_spacing(1)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
} 