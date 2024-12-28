use crate::ui::common::*;
use crate::rpc::BlockDetails;
use num_format::{Locale, ToFormattedString};

pub enum BlockSearchMode {
    Latest,     // Zeigt den neuesten Block
    Custom(String),  // Zeigt einen benutzerdefinierten Block (Hash oder Höhe)
}

pub fn render(
    block: &BlockDetails,
    search_mode: &BlockSearchMode,
    is_input_active: bool,
) -> Paragraph<'static> {
    let dt = DateTime::<Utc>::from_timestamp(block.timestamp, 0)
        .unwrap_or_default();

    let content = vec![
        Line::from(vec![
            Span::styled("📦 Block Information", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(""),
        // Suchleiste
        Line::from(vec![
            Span::styled("🔍 ", Style::default().fg(Color::Yellow)),
            Span::styled("Block suchen: ", Style::default().fg(Color::Cyan)),
            match search_mode {
                BlockSearchMode::Latest => Span::styled(
                    if is_input_active {
                        ">"
                    } else {
                        "[Aktueller Block]"
                    },
                    Style::default().fg(if is_input_active { Color::Yellow } else { Color::Green })
                ),
                BlockSearchMode::Custom(input) => Span::styled(
                    if is_input_active {
                        format!(">{}█", input)
                    } else {
                        format!("[{}]", input)
                    },
                    Style::default().fg(Color::Yellow)
                ),
            },
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Höhe: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                block.height.to_formatted_string(&Locale::de),
                Style::default().fg(Color::White)
            ),
        ]),
        Line::from(vec![
            Span::styled("Hash: ", Style::default().fg(Color::Cyan)),
            Span::styled(block.hash.clone(), Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("Zeit: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                dt.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
                Style::default().fg(Color::White)
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("🔍 Block Details", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(vec![
            Span::styled("Transaktionen: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                block.tx_count.to_formatted_string(&Locale::de),
                Style::default().fg(Color::White)
            ),
        ]),
        Line::from(vec![
            Span::styled("Größe: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{} Bytes", block.size.to_formatted_string(&Locale::de)),
                Style::default().fg(Color::White)
            ),
        ]),
        Line::from(vec![
            Span::styled("Gewicht: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{} WU", block.weight.to_formatted_string(&Locale::de)),
                Style::default().fg(Color::White)
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("🔧 Technische Details", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(vec![
            Span::styled("Version: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                block.version.to_string(),
                Style::default().fg(Color::White)
            ),
        ]),
        Line::from(vec![
            Span::styled("Merkle Root: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                block.merkle_root.clone(),
                Style::default().fg(Color::White)
            ),
        ]),
        Line::from(vec![
            Span::styled("Bits: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                block.bits.clone(),
                Style::default().fg(Color::White)
            ),
        ]),
        Line::from(vec![
            Span::styled("Nonce: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                block.nonce.to_formatted_string(&Locale::de),
                Style::default().fg(Color::White)
            ),
        ]),
    ];

    Paragraph::new(content)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title(" Block Details "))
} 