// Für Header, Footer und andere wiederverwendbare Komponenten
use super::common::*;
use std::time::Duration;
use ratatui::widgets::Tabs;
use ratatui::prelude::Alignment;

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

pub fn create_footer(update_interval: Duration, is_updating: bool, spinner_state: usize) -> Paragraph<'static> {
    let spinner = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    
    let status_text = if is_updating {
        format!("{} Aktualisiere...", spinner[spinner_state])
    } else {
        format!("Letzte Aktualisierung: vor {}s", update_interval.as_secs())
    };

    Paragraph::new(vec![
        Line::from(vec![
            Span::styled("H", Style::default().fg(Color::Yellow)),
            Span::raw(": Hilfe | "),
            Span::styled("Q", Style::default().fg(Color::Yellow)),
            Span::raw(": Beenden | "),
            Span::styled(status_text, Style::default().fg(Color::Blue))
        ])
    ])
    .alignment(Alignment::Center)
    .block(Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray)))
}

pub fn create_tabs(current_tab: &Tab) -> Tabs<'static> {
    let titles = vec![
        "Dashboard",
        "Block Details",
        "TX Details",
        "Address Details",
        "Mempool",
        "Network",
        "Peer List",
        "Mining",
        "Security",
    ];
    
    let tabs = titles.iter().map(|t| {
        Line::from(Span::styled(*t, Style::default().fg(Color::White)))
    }).collect();

    Tabs::new(tabs)
        .block(Block::default().borders(Borders::ALL).title(" Navigation "))
        .select(match current_tab {
            Tab::Dashboard => 0,
            Tab::BlockDetails => 1,
            Tab::TxDetails => 2,
            Tab::AddressDetails => 3,
            Tab::Mempool => 4,
            Tab::Network => 5,
            Tab::PeerList => 6,
            Tab::Mining => 7,
            Tab::Security => 8,
        })
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD))
} 