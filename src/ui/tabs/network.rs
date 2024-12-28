use crate::ui::common::*;
use crate::rpc::PeerInfo;

pub fn render(connections: u64, _network: String, verification_progress: f64, peers: &[PeerInfo]) -> Paragraph<'static> {
    Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Netzwerk Status", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Verbindungen: ", Style::default().fg(Color::Cyan)),
            Span::raw(connections.to_string()),
        ]),
    ])
    .block(Block::default()
        .borders(Borders::ALL)
        .title(" Netzwerk "))
} 