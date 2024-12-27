use crate::ui::common::*;

pub fn render() -> Paragraph<'static> {
    Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Mining Status", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
    ])
    .block(Block::default()
        .borders(Borders::ALL)
        .title(" Mining "))
} 