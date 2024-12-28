use crate::ui::common::*;
use crate::rpc::MempoolStats;
use num_format::{Locale, ToFormattedString};

pub fn render(stats: &MempoolStats, is_loading: bool) -> Paragraph<'static> {
    let title = if is_loading {
        " Mempool 🔄 "  // Lade-Symbol
    } else {
        " Mempool "
    };

    let content = vec![
        // Titel
        Line::from(vec![
            Span::styled("💭 Mempool Status", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(""),
        
        // Allgemeine Info
        Line::from(vec![
            Span::styled("Transaktionen: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                stats.tx_count.to_formatted_string(&Locale::de),
                Style::default().fg(Color::White)
            ),
        ]),
        Line::from(vec![
            Span::styled("Größe: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{:.2} MB", (stats.size as f64 / 1_000_000.0)),
                Style::default().fg(Color::White)
            ),
        ]),
        Line::from(""),
        
        // Gebühren-Header vereinfachen
        Line::from(vec![
            Span::styled("📊 Transaction Fees", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(""),
        
        // No Priority
        Line::from(vec![
            Span::styled("No Priority    ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{:>3.0} sat/vB", stats.no_priority.rate),
                Style::default().fg(Color::White)
            ),
            Span::styled(
                format!("  ${:.2}", stats.no_priority.usd_price),
                Style::default().fg(Color::DarkGray)
            ),
        ]),
        
        // Low Priority
        Line::from(vec![
            Span::styled("Low Priority   ", Style::default().fg(Color::Red)),
            Span::styled(
                format!("{:>3.0} sat/vB", stats.low_priority.rate),
                Style::default().fg(Color::White)
            ),
            Span::styled(
                format!("  ${:.2}", stats.low_priority.usd_price),
                Style::default().fg(Color::DarkGray)
            ),
        ]),
        
        // Medium Priority
        Line::from(vec![
            Span::styled("Medium Priority", Style::default().fg(Color::Yellow)),
            Span::styled(
                format!("{:>3.0} sat/vB", stats.medium_priority.rate),
                Style::default().fg(Color::White)
            ),
            Span::styled(
                format!("  ${:.2}", stats.medium_priority.usd_price),
                Style::default().fg(Color::DarkGray)
            ),
        ]),
        // High Priority
        Line::from(vec![
            Span::styled("High Priority  ", Style::default().fg(Color::Green)),
            Span::styled(
                format!("{:>3.0} sat/vB", stats.high_priority.rate),
                Style::default().fg(Color::White)
            ),
            Span::styled(
                format!("  ${:.2}", stats.high_priority.usd_price),
                Style::default().fg(Color::DarkGray)
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("💾 Speichernutzung", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(vec![
            Span::styled("Aktuell: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{} MB", (stats.memory_usage as f64 / 1_000_000.0).round()),
                Style::default().fg(Color::White)
            ),
        ]),
        Line::from(vec![
            Span::styled("Maximum: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{} MB", (stats.max_memory as f64 / 1_000_000.0).round()),
                Style::default().fg(Color::White)
            ),
        ]),
    ];

    Paragraph::new(content)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title(title))
} 