use crate::ui::common::*;
use num_format::{Locale, ToFormattedString};
use ratatui::prelude::Alignment;

pub fn render(
    network: &str,
    connections: u64,
    blocks: u64,
    headers: u64,
    difficulty: f64,
    chain_work: String,
    verification_progress: f64,
    initial_block_download: bool,
    size_on_disk: u64,
    pruned: bool,
    mempool_size: u64,
    warnings: &str,
) -> Paragraph<'static> {
    // Konstanten für bessere Lesbarkeit
    const CRITICAL_CONNECTIONS: u64 = 4;
    const WARNING_CONNECTIONS: u64 = 8;
    const SYNC_COMPLETE_THRESHOLD: f64 = 0.99;
    
    // Farbdefinitionen für konsistenteres Aussehen
    let title_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
    let label_style = Style::default().fg(Color::Cyan);
    let value_style = Style::default().fg(Color::White);
    let success_style = Style::default().fg(Color::Green);
    let warning_style = Style::default().fg(Color::Yellow);
    let error_style = Style::default().fg(Color::Red);
    
    // Verbindungsstatus-Anzeige verbessern
    let connection_style = if connections > WARNING_CONNECTIONS {
        success_style
    } else if connections > CRITICAL_CONNECTIONS {
        warning_style
    } else {
        error_style.add_modifier(Modifier::BOLD)
    };
    
    let connection_warning = if connections < CRITICAL_CONNECTIONS {
        Span::styled(" (Zu wenige Verbindungen!)", error_style)
    } else {
        Span::raw("")
    };

    // Synchronisationsstatus besser darstellen
    let sync_style = if verification_progress >= SYNC_COMPLETE_THRESHOLD {
        success_style
    } else {
        warning_style
    };
    
    let ibd_style = if initial_block_download {
        warning_style
    } else {
        success_style
    };

    // Pruning-Status
    let pruned_style = if pruned {
        warning_style
    } else {
        success_style
    };

    // Inhalt erstellen
    let mut content = vec![
        // Netzwerk-Header mit verbesserten Icons und Layout
        Line::from(vec![
            Span::styled("⚡ ", Style::default().fg(Color::Yellow)),
            Span::styled(format!("Netzwerk: {} ", network), 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" | "),
            Span::styled("🔌 ", Style::default().fg(Color::Yellow)),
            Span::styled(
                format!("{} Peers", connections.to_formatted_string(&Locale::de)),
                connection_style
            ),
            connection_warning
        ]),
        Line::from(""),
        
        // Blockchain-Sektion
        Line::from(vec![
            Span::styled("📦 Blockchain Status", title_style),
        ]),
        Line::from(vec![
            Span::styled("Blöcke: ", label_style),
            Span::styled(blocks.to_formatted_string(&Locale::de), value_style),
            Span::raw(" | "),
            Span::styled("Headers: ", label_style),
            Span::styled(headers.to_formatted_string(&Locale::de), value_style),
            if headers > blocks {
                Span::styled(
                    format!(" ({} ausstehend)", 
                        (headers - blocks).to_formatted_string(&Locale::de)),
                    warning_style
                )
            } else {
                Span::raw("")
            }
        ]),
        Line::from(vec![
            Span::styled("Difficulty: ", label_style),
            Span::styled(format!("{:.2e}", difficulty), value_style),
            Span::raw(" | "),
            Span::styled("Chain Work: ", label_style),
            Span::styled(chain_work.clone(), value_style),
        ]),
        Line::from(""),
        
        // Synchronisations-Sektion
        Line::from(vec![
            Span::styled("🔄 Synchronisation", title_style),
        ]),
        Line::from(vec![
            Span::styled("Status: ", label_style),
            Span::styled(
                format!("{:.2}%", verification_progress * 100.0),
                sync_style,
            ),
            Span::raw(" | "),
            Span::styled("IBD: ", label_style),
            Span::styled(
                if initial_block_download { "Aktiv" } else { "Abgeschlossen" },
                ibd_style,
            ),
        ]),
        Line::from(vec![
            Span::styled("Speicherplatz: ", label_style),
            Span::styled(format_size(size_on_disk), value_style),
            Span::raw(" | "),
            Span::styled("Pruned: ", label_style),
            Span::styled(
                if pruned { "Ja" } else { "Nein" },
                pruned_style,
            ),
        ]),
        Line::from(""),
        
        // Mempool-Sektion
        Line::from(vec![
            Span::styled("💭 Mempool", title_style),
            Span::raw(" (lokal)"),
        ]),
        Line::from(vec![
            Span::styled("Transaktionen: ", label_style),
            Span::styled(mempool_size.to_formatted_string(&Locale::de), value_style),
        ]),
    ];

    // Warnungs-Sektion (nur wenn nötig)
    if !warnings.is_empty() {
        content.extend(vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("⚠️ Warnungen: ", 
                    error_style.add_modifier(Modifier::BOLD)),
                Span::styled(warnings.to_string(), error_style),
            ]),
        ]);
    }

    // Paragraph erstellen mit verbessertem Stil
    Paragraph::new(content)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title(Span::styled(" Dashboard Übersicht ", 
                   Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))))
        .alignment(Alignment::Left)
}

fn format_size(bytes: u64) -> String {
    const KB: f64 = 1_024.0;
    const MB: f64 = KB * 1_024.0;
    const GB: f64 = MB * 1_024.0;
    const TB: f64 = GB * 1_024.0;

    let bytes_f64 = bytes as f64;
    
    if bytes_f64 >= TB {
        format!("{:.2} TB", bytes_f64 / TB)
    } else if bytes_f64 >= GB {
        format!("{:.2} GB", bytes_f64 / GB)
    } else if bytes_f64 >= MB {
        format!("{:.2} MB", bytes_f64 / MB)
    } else if bytes_f64 >= KB {
        format!("{:.2} KB", bytes_f64 / KB)
    } else {
        format!("{} Bytes", bytes)
    }
}