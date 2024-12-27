// Für Header, Footer und andere wiederverwendbare Komponenten
use super::common::*;
use std::time::Duration;
use ratatui::widgets::Tabs;

pub fn create_header(version: u64) -> Paragraph<'static> {
    let version_str = format!("Bitcoin Core v{}.{}.{}",
        version / 10000,
        (version % 10000) / 100,
        version % 100
    );
    Paragraph::new(version_str)
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title(" Node Info "))
}

pub fn create_footer(
    update_interval: Duration,
    is_updating: bool,
    spinner_state: usize,
) -> Paragraph<'static> {
    const SPINNER: [&str; 4] = ["⠋", "⠙", "⠹", "⠸"];
    
    let update_status = if is_updating {
        format!("{} Aktualisiere...", SPINNER[spinner_state])
    } else {
        String::from("✓ Bereit")
    };

    let content = vec![
        Line::from(vec![
            Span::raw(" ["),
            Span::styled("Q", Style::default().fg(Color::Yellow)),
            Span::raw("] Beenden | ["),
            Span::styled("R", Style::default().fg(Color::Yellow)),
            Span::raw("] Aktualisieren | ["),
            Span::styled("H", Style::default().fg(Color::Yellow)),
            Span::raw("] Hilfe | ["),
            Span::styled("+/-", Style::default().fg(Color::Yellow)),
            Span::raw(format!("] Update-Intervall: {}s | ", update_interval.as_secs())),
            Span::styled(
                update_status,
                Style::default().fg(if is_updating { Color::Yellow } else { Color::Green })
            ),
        ])
    ];

    Paragraph::new(content)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)))
}

pub fn create_tabs(current_tab: &Tab) -> Tabs<'static> {
    let titles = vec![
        "Übersicht",
        "Block Details",
        "Mempool",
        "Netzwerk",
        "Peer Liste",
        "Mining",
        "Security",
        "Explorer",
    ];

    let tabs = titles.iter().map(|t| {
        Line::from(vec![
            Span::raw(*t)
        ])
    }).collect();

    Tabs::new(tabs)
        .block(Block::default().borders(Borders::ALL))
        .select(match current_tab {
            Tab::Overview => 0,
            Tab::BlockDetails => 1,
            Tab::Mempool => 2,
            Tab::Network => 3,
            Tab::PeerList => 4,
            Tab::Mining => 5,
            Tab::Security => 6,
            Tab::Explorer => 7,
        })
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD))
} 