use crate::ui::common::*;
use num_format::{Locale, ToFormattedString};

pub fn render(
    difficulty: f64,
    blocks_until_adjustment: i64,
    estimated_hashrate: f64,
    next_difficulty_estimate: f64,
) -> Paragraph<'static> {
    let content = vec![
        Line::from(vec![
            Span::styled("⛏️  Mining Status", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("📊 Aktuelle Difficulty", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(vec![
            Span::styled("Wert: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{:.2e}", difficulty),
                Style::default().fg(Color::White)
            ),
        ]),
        Line::from(vec![
            Span::styled("Dezimal: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                (difficulty.round() as u64).to_formatted_string(&Locale::de),
                Style::default().fg(Color::White)
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("🔄 Nächste Anpassung", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(vec![
            Span::styled("Blöcke: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                blocks_until_adjustment.to_formatted_string(&Locale::de),
                Style::default().fg(Color::White)
            ),
            Span::raw(" ("),
            Span::styled(
                format!("~{:.1} Tage", blocks_until_adjustment as f64 * 10.0 / (60.0 * 24.0)),
                Style::default().fg(Color::DarkGray)
            ),
            Span::raw(")"),
        ]),
        Line::from(vec![
            Span::styled("Geschätzte Änderung: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{:+.2}%", (next_difficulty_estimate / difficulty - 1.0) * 100.0),
                Style::default().fg(if next_difficulty_estimate > difficulty {
                    Color::Red
                } else {
                    Color::Green
                })
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("⚡ Netzwerk-Hashrate", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(vec![
            Span::styled("Aktuell: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format_hashrate(estimated_hashrate),
                Style::default().fg(Color::White)
            ),
        ]),
    ];

    Paragraph::new(content)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title(" Mining "))
}

fn format_hashrate(h: f64) -> String {
    if h >= 1e18 {
        format!("{:.2} EH/s", h / 1e18)
    } else if h >= 1e15 {
        format!("{:.2} PH/s", h / 1e15)
    } else if h >= 1e12 {
        format!("{:.2} TH/s", h / 1e12)
    } else if h >= 1e9 {
        format!("{:.2} GH/s", h / 1e9)
    } else if h >= 1e6 {
        format!("{:.2} MH/s", h / 1e6)
    } else if h >= 1e3 {
        format!("{:.2} KH/s", h / 1e3)
    } else {
        format!("{:.2} H/s", h)
    }
} 