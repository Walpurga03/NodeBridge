use crate::ui::common::*;
use num_format::{Locale, ToFormattedString};

pub fn render(
    network: &str,
    connections: u64,
    blocks: u64,
    headers: u64,
    difficulty: f64,
    chain_work: String,
    verification_progress: f64,
    initial_block_download: bool,
    size_on_disk: u64,
    pruned: bool,
    mempool_size: u64,
    warnings: &str,
) -> Paragraph<'static> {
    let mut content = vec![
        Line::from(vec![
            Span::styled("⚡ ", Style::default().fg(Color::Yellow)),
            Span::styled(format!("Netzwerk: {}", network), 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" | "),
            Span::styled("🔌 ", Style::default().fg(Color::Yellow)),
            Span::styled(format!("{} Peers", connections.to_formatted_string(&Locale::de)),
                Style::default().fg(if connections > 8 { Color::Green } else { Color::Yellow })),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("📦 Blockchain Status", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("Blöcke: ", Style::default().fg(Color::Cyan)),
            Span::styled(blocks.to_formatted_string(&Locale::de), 
                Style::default().fg(Color::White)),
            Span::raw(" | "),
            Span::styled("Headers: ", Style::default().fg(Color::Cyan)),
            Span::styled(headers.to_formatted_string(&Locale::de), 
                Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("Difficulty: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{:.2e}", difficulty),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled("Chain Work: ", Style::default().fg(Color::Cyan)),
            Span::styled(chain_work, Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("🔄 Synchronisation", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("Status: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{:.2}%", verification_progress * 100.0),
                Style::default().fg(if verification_progress >= 0.99 {
                    Color::Green
                } else {
                    Color::Yellow
                }),
            ),
            Span::raw(" | "),
            Span::styled("IBD: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                if initial_block_download { "Aktiv" } else { "Abgeschlossen" },
                Style::default().fg(if initial_block_download {
                    Color::Yellow
                } else {
                    Color::Green
                }),
            ),
        ]),
        Line::from(vec![
            Span::styled("Speicherplatz: ", Style::default().fg(Color::Cyan)),
            Span::styled(format_size(size_on_disk), Style::default().fg(Color::White)),
            Span::raw(" | "),
            Span::styled("Pruned: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                if pruned { "Ja" } else { "Nein" },
                Style::default().fg(if pruned {
                    Color::Yellow
                } else {
                    Color::Green
                }),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("💭 Mempool", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("Transaktionen: ", Style::default().fg(Color::Cyan)),
            Span::styled(mempool_size.to_formatted_string(&Locale::de), 
                Style::default().fg(Color::White)),
        ]),
    ];

    if !warnings.is_empty() {
        content.extend(vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("⚠️ Warnungen: ", 
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::styled(warnings.to_string(), Style::default().fg(Color::Red)),
            ]),
        ]);
    }

    Paragraph::new(content)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title(" Node Information "))
}

fn format_size(bytes: u64) -> String {
    if bytes >= 1_000_000_000_000 {
        format!("{:.2} TB", bytes as f64 / 1_000_000_000_000.0)
    } else if bytes >= 1_000_000_000 {
        format!("{:.2} GB", bytes as f64 / 1_000_000_000.0)
    } else {
        format!("{:.2} MB", bytes as f64 / 1_000_000.0)
    }
} 