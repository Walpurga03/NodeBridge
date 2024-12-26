// Für Header, Footer und andere wiederverwendbare Komponenten
use super::common::*;
use std::time::Duration;

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

pub fn create_tabs(current_tab: &Tab) -> Paragraph<'static> {
    let tabs = vec![
        (Tab::Overview, "Übersicht"),
        (Tab::BlockDetails, "Block Details"),
        (Tab::Mempool, "Mempool"),
        (Tab::Network, "Netzwerk"),
    ];

    let content = Line::from(
        tabs.iter()
            .flat_map(|(tab, title)| {
                vec![
                    Span::raw("["),
                    Span::styled(
                        *title,
                        Style::default().fg(
                            if tab == current_tab {
                                Color::Green
                            } else {
                                Color::Gray
                            }
                        )
                    ),
                    Span::raw("] "),
                ]
            })
            .collect::<Vec<_>>()
    );

    Paragraph::new(vec![content])
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title(" Navigation "))
} 