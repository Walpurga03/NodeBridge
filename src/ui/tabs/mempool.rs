use crate::ui::common::*;
use crate::rpc::MempoolInfo;

pub fn render(info: &MempoolInfo) -> Paragraph<'static> {
    Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Mempool Status", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Transaktionen: ", Style::default().fg(Color::Cyan)),
            Span::raw(info.size.to_string()),
        ]),
    ])
    .block(Block::default()
        .borders(Borders::ALL)
        .title(" Mempool "))
} 