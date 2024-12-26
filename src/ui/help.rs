// Für die Hilfe-Ansicht
use super::common::*;

pub fn create_help() -> Paragraph<'static> {
    let content = vec![
        Line::from(vec![
            Span::styled("Tastatur-Befehle", 
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::Black)),
        ]),
        Line::from(Span::styled("", Style::default().bg(Color::Black))),
        Line::from(vec![
            Span::styled("Navigation", 
                Style::default()
                    .fg(Color::Cyan)
                    .bg(Color::Black)),
        ]),
        Line::from(vec![
            Span::styled(" [1-4] ", 
                Style::default().bg(Color::Black)),
            Span::styled("Ansichten wechseln", 
                Style::default()
                    .fg(Color::Gray)
                    .bg(Color::Black)),
        ]),
        Line::from(vec![
            Span::styled(" [h] ", 
                Style::default().bg(Color::Black)),
            Span::styled("Hilfe ein/ausblenden", 
                Style::default()
                    .fg(Color::Gray)
                    .bg(Color::Black)),
        ]),
        Line::from(vec![
            Span::styled(" [ESC] ", 
                Style::default().bg(Color::Black)),
            Span::styled("Hilfe ausblenden", 
                Style::default()
                    .fg(Color::Gray)
                    .bg(Color::Black)),
        ]),
        Line::from(Span::styled("", Style::default().bg(Color::Black))),
        Line::from(vec![
            Span::styled("Updates", 
                Style::default()
                    .fg(Color::Cyan)
                    .bg(Color::Black)),
        ]),
        Line::from(vec![
            Span::styled(" [r] ", 
                Style::default().bg(Color::Black)),
            Span::styled("Manuelles Update", 
                Style::default()
                    .fg(Color::Gray)
                    .bg(Color::Black)),
        ]),
        Line::from(vec![
            Span::styled(" [+] ", 
                Style::default().bg(Color::Black)),
            Span::styled("Update-Intervall erhöhen (max: 60s)", 
                Style::default()
                    .fg(Color::Gray)
                    .bg(Color::Black)),
        ]),
        Line::from(vec![
            Span::styled(" [-] ", 
                Style::default().bg(Color::Black)),
            Span::styled("Update-Intervall verringern (min: 1s)", 
                Style::default()
                    .fg(Color::Gray)
                    .bg(Color::Black)),
        ]),
        Line::from(Span::styled("", Style::default().bg(Color::Black))),
        Line::from(vec![
            Span::styled("Status-Anzeigen", 
                Style::default()
                    .fg(Color::Cyan)
                    .bg(Color::Black)),
        ]),
        Line::from(vec![
            Span::styled(" ✓ ", 
                Style::default()
                    .fg(Color::Green)
                    .bg(Color::Black)),
            Span::styled("Bereit - Warte auf nächstes Update", 
                Style::default()
                    .fg(Color::Gray)
                    .bg(Color::Black)),
        ]),
        Line::from(vec![
            Span::styled(" ⠋⠙⠹⠸ ", 
                Style::default()
                    .fg(Color::Yellow)
                    .bg(Color::Black)),
            Span::styled("Aktualisiere Daten...", 
                Style::default()
                    .fg(Color::Gray)
                    .bg(Color::Black)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Mempool Details:", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(""),
        Line::from("Die Mempool-Ansicht zeigt Informationen über unbestätigte Transaktionen:"),
        Line::from(""),
        Line::from(vec![
            Span::styled("• Transaktionen: ", Style::default().fg(Color::Cyan)),
            Span::styled("Gesamtzahl der unbestätigten Transaktionen", Style::default()),
        ]),
        Line::from(vec![
            Span::styled("• Größe: ", Style::default().fg(Color::Cyan)),
            Span::styled("Gesamtgröße aller Transaktionen in MB", Style::default()),
        ]),
        Line::from(vec![
            Span::styled("• Speichernutzung: ", Style::default().fg(Color::Cyan)),
            Span::styled("Aktueller Mempool-Speicherverbrauch", Style::default()),
        ]),
        Line::from(""),
        Line::from("Gebührenverteilung (sat/vB):"),
        Line::from(vec![
            Span::styled("• 1-3: ", Style::default().fg(Color::LightGreen)),
            Span::styled("Sehr niedrige Priorität, lange Wartezeit", Style::default()),
        ]),
        Line::from(vec![
            Span::styled("• 4-6: ", Style::default().fg(Color::LightGreen)),
            Span::styled("Niedrige Priorität", Style::default()),
        ]),
        Line::from(vec![
            Span::styled("• 7-10: ", Style::default().fg(Color::Green)),
            Span::styled("Mittlere Priorität", Style::default()),
        ]),
        Line::from(vec![
            Span::styled("• 11-15: ", Style::default().fg(Color::Yellow)),
            Span::styled("Hohe Priorität", Style::default()),
        ]),
        Line::from(vec![
            Span::styled("• 16-20: ", Style::default().fg(Color::LightRed)),
            Span::styled("Sehr hohe Priorität", Style::default()),
        ]),
        Line::from(vec![
            Span::styled("• 21+: ", Style::default().fg(Color::Red)),
            Span::styled("Express-Priorität, nächster Block", Style::default()),
        ]),
        Line::from(""),
        Line::from("Balkendiagramm:"),
        Line::from(vec![
            Span::styled("• Länge: ", Style::default().fg(Color::Cyan)),
            Span::styled("Prozentualer Anteil der Transaktionen", Style::default()),
        ]),
        Line::from(vec![
            Span::styled("• Farbe: ", Style::default().fg(Color::Cyan)),
            Span::styled("Indikator für die Priorität/Geschwindigkeit", Style::default()),
        ]),
        Line::from(vec![
            Span::styled("• Zahlen: ", Style::default().fg(Color::Cyan)),
            Span::styled("Prozent und absolute Anzahl der Transaktionen", Style::default()),
        ]),
    ];

    Paragraph::new(content)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title(" Hilfe "))
} 