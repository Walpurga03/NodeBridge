use crate::ui::common::*;

pub fn render() -> Paragraph<'static> {
    Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Blockchain Explorer", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
    ])
    .block(Block::default()
        .borders(Borders::ALL)
        .title(" Explorer "))
} 