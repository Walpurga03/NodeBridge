use crate::ui::common::*;

pub fn render(height: u64, hash: String, timestamp: i64) -> Paragraph<'static> {
    Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Block Details", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Höhe: ", Style::default().fg(Color::Cyan)),
            Span::raw(height.to_string()),
        ]),
        Line::from(vec![
            Span::styled("Hash: ", Style::default().fg(Color::Cyan)),
            Span::raw(hash),
        ]),
        Line::from(vec![
            Span::styled("Zeit: ", Style::default().fg(Color::Cyan)),
            Span::raw(timestamp.to_string()),
        ]),
    ])
    .block(Block::default()
        .borders(Borders::ALL)
        .title(" Block Details "))
} 