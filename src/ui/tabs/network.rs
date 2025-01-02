use crate::ui::common::*;
use crate::rpc::PeerInfo;
use num_format::{Locale, ToFormattedString};
use log::{info, warn, error};

pub fn render(connections: u64, network: String, verification_progress: f64, peers: &[PeerInfo]) -> Paragraph<'static> {
    // Peer-Statistiken berechnen
    let inbound_count = peers.iter().filter(|p| p.inbound).count();
    let outbound_count = peers.len() - inbound_count;
    
    // Latenz-Statistiken
    let mut min_latency: f64 = f64::MAX;
    let mut max_latency: f64 = 0.0;
    let mut avg_latency: f64 = 0.0;
    let mut onion_count = 0;

    for peer in peers {
        min_latency = min_latency.min(peer.latency);
        max_latency = max_latency.max(peer.latency);
        avg_latency += peer.latency;
        
        if peer.addr.contains(".onion") {
            onion_count += 1;
        }
    }

    avg_latency = if !peers.is_empty() {
        avg_latency / peers.len() as f64
    } else {
        0.0
    };

    let content = vec![
        Line::from(vec![
            Span::styled("⚡ Netzwerk Status", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Netzwerk: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                network,
                Style::default().fg(Color::White)
            ),
        ]),
        Line::from(vec![
            Span::styled("Synchronisation: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{:.2}%", verification_progress * 100.0),
                Style::default().fg(if verification_progress >= 0.99 {
                    Color::Green
                } else {
                    Color::Yellow
                })
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("🔌 Verbindungen", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(vec![
            Span::styled("Gesamt: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                connections.to_formatted_string(&Locale::de),
                Style::default().fg(if connections >= 8 { Color::Green } else { Color::Yellow })
            ),
        ]),
        Line::from(vec![
            Span::styled("Eingehend: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                inbound_count.to_formatted_string(&Locale::de),
                Style::default().fg(Color::White)
            ),
        ]),
        Line::from(vec![
            Span::styled("Ausgehend: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                outbound_count.to_formatted_string(&Locale::de),
                Style::default().fg(Color::White)
            ),
        ]),
        Line::from(vec![
            Span::styled("Tor (Onion): ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{} ({:.1}%)", 
                    onion_count,
                    (onion_count as f64 / peers.len() as f64) * 100.0
                ),
                Style::default().fg(Color::White)
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("📊 Latenz (ms)", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(vec![
            Span::styled("Minimum: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{:.2}", min_latency),
                Style::default().fg(if min_latency < 100.0 { Color::Green } else { Color::Yellow })
            ),
        ]),
        Line::from(vec![
            Span::styled("Maximum: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{:.2}", max_latency),
                Style::default().fg(if max_latency > 500.0 { Color::Red } else { Color::Yellow })
            ),
        ]),
        Line::from(vec![
            Span::styled("Durchschnitt: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{:.2}", avg_latency),
                Style::default().fg(if avg_latency < 200.0 { Color::Green } else { Color::Yellow })
            ),
        ]),
    ];

    Paragraph::new(content)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title(" Netzwerk "))
}

#[allow(dead_code)]
pub fn display_network_info() {
    info!("Netzwerk-Informationen werden abgerufen.");
    // Netzwerk-Info Logik
    warn!("Dies ist eine Warnmeldung für das Netzwerk-Modul.");
    error!("Dies ist eine Fehlermeldung für das Netzwerk-Modul.");
} 