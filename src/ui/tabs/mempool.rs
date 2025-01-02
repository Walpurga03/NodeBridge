use crate::rpc::MempoolStats;
use num_format::{Locale, ToFormattedString};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    prelude::Alignment,
};

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
    ];

    Paragraph::new(content)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title(title))
}

#[allow(dead_code)]
pub fn render_help() -> Paragraph<'static> {
    let text = vec![
        Line::from(vec![
            Span::styled("📊 Mempool-Übersicht Hilfe", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Allgemeine Statistiken:", Style::default().fg(Color::Cyan))
        ]),
        Line::from("• Transaktionen: Gesamtzahl der unbestätigten Transaktionen im Mempool"),
        Line::from("• Größe: Aktuelle Größe des Mempools in Megabyte (MB)"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Gebührenkategorien:", Style::default().fg(Color::Cyan))
        ]),
        Line::from("1. No Priority (1 sat/vB):"),
        Line::from("   - Sehr niedrige Priorität"),
        Line::from("   - Bestätigung ungewiss"),
        Line::from("   - Günstigste Option (~$0.11)"),
        Line::from(""),
        Line::from("2. Low Priority (2 sat/vB):"),
        Line::from("   - Niedrige Priorität"),
        Line::from("   - Bestätigung in 1-3 Tagen"),
        Line::from("   - Kostengünstige Option (~$0.21)"),
        Line::from(""),
        Line::from("3. Medium Priority (3 sat/vB):"),
        Line::from("   - Mittlere Priorität"),
        Line::from("   - Bestätigung in 12-24 Stunden"),
        Line::from("   - Moderate Option (~$0.32)"),
        Line::from(""),
        Line::from("4. High Priority (>3 sat/vB):"),
        Line::from("   - Hohe Priorität"),
        Line::from("   - Nächste 1-6 Blöcke"),
        Line::from("   - Express-Option (~$1.70)"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Anzeige-Details:", Style::default().fg(Color::Cyan))
        ]),
        Line::from("• sat/vB: Gebühr in Satoshi pro virtuellem Byte"),
        Line::from("• USD: Geschätzter Preis für eine durchschnittliche Transaktion (250 Bytes)"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Datenquelle:", Style::default().fg(Color::Cyan))
        ]),
        Line::from("• Live-Daten von mempool.space"),
        Line::from("• Preise basieren auf aktuellem Bitcoin-Kurs"),
    ];

    Paragraph::new(text)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title(" Mempool Hilfe "))
        .alignment(Alignment::Left)
} 