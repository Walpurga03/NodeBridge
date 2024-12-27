// Für die Hilfe-Ansicht
use super::common::*;

pub fn create_help(current_tab: &Tab) -> Paragraph<'static> {
    let content = match current_tab {
        Tab::Overview => vec![
            Line::from(vec![
                Span::styled("Übersicht & Block Info", Style::default().fg(Color::Yellow)),
            ]),
            Line::from(""),
            Line::from("Tastenbelegung:"),
            Line::from(" [R] Aktualisieren"),
            Line::from(" [H] Hilfe ausblenden"),
            Line::from(""),
            Line::from("Anzeigen:"),
            Line::from(" • Blockhöhe und Hash"),
            Line::from(" • Netzwerk-Typ und Verbindungen"),
            Line::from(" • Synchronisationsstatus"),
            Line::from(" • Mining-Difficulty (NEU)"),
            Line::from(" • Hashrate (NEU)"),
        ],
        Tab::BlockDetails => vec![
            Line::from(vec![
                Span::styled("Block Details", Style::default().fg(Color::Yellow)),
            ]),
            Line::from(""),
            Line::from(" • Aktuelle Blockhöhe"),
            Line::from(" • Block Hash"),
            Line::from(" • Zeitstempel"),
        ],
        Tab::Mempool => vec![
            Line::from(vec![
                Span::styled("Mempool Status", Style::default().fg(Color::Yellow)),
            ]),
            Line::from(""),
            Line::from("Tastenbelegung:"),
            Line::from(" [R] Aktualisieren"),
            Line::from(" [H] Hilfe ausblenden"),
            Line::from(""),
            Line::from("Statistiken:"),
            Line::from(" • Transaktionsanzahl und Größe"),
            Line::from(" • Gebührenverteilung (sat/vB)"),
            Line::from(" • Größenverteilung der Transaktionen"),
            Line::from(" • Altersverteilung im Mempool"),
        ],
        Tab::Network => vec![
            Line::from(vec![
                Span::styled("Netzwerk & Peers", Style::default().fg(Color::Yellow)),
            ]),
            Line::from(""),
            Line::from("Tastenbelegung:"),
            Line::from(" [R] Aktualisieren"),
            Line::from(" [H] Hilfe ausblenden"),
            Line::from(""),
            Line::from("Verbindungen:"),
            Line::from(" • Gesamtzahl und Onion-Anteil"),
            Line::from(" • Beste/Schlechteste Latenz"),
            Line::from(""),
            Line::from("Latenz-Werte:"),
            Line::from(vec![
                Span::raw(" • "),
                Span::styled("< 100ms", Style::default().fg(Color::Green)),
                Span::raw(" = sehr gut"),
            ]),
            Line::from(vec![
                Span::raw(" • "),
                Span::styled("100-500ms", Style::default().fg(Color::Yellow)),
                Span::raw(" = normal"),
            ]),
            Line::from(vec![
                Span::raw(" • "),
                Span::styled("> 500ms", Style::default().fg(Color::Red)),
                Span::raw(" = langsam"),
            ]),
        ],
        Tab::PeerList => vec![
            Line::from(vec![
                Span::styled("Peer Liste", Style::default().fg(Color::Yellow)),
            ]),
            Line::from(""),
            Line::from(" • Liste aller verbundenen Peers"),
            Line::from(" • Sortiert nach Latenz"),
            Line::from(" • Onion/Clearnet Status"),
        ],
        Tab::Mining => vec![
            Line::from(vec![
                Span::styled("Mining Status", Style::default().fg(Color::Yellow)),
            ]),
            Line::from(""),
            Line::from("Tastenbelegung:"),
            Line::from(" [R] Aktualisieren"),
            Line::from(" [H] Hilfe ausblenden"),
            Line::from(""),
            Line::from("Informationen:"),
            Line::from(" • Aktuelle Difficulty"),
            Line::from(" • Netzwerk-Hashrate"),
            Line::from(" • Zeit bis Difficulty-Anpassung"),
            Line::from(" • Mining-Pool Verteilung"),
        ],
        Tab::Security => vec![
            Line::from(vec![
                Span::styled("Sicherheitsstatus", Style::default().fg(Color::Yellow)),
            ]),
            Line::from(""),
            Line::from("Tastenbelegung:"),
            Line::from(" [R] Aktualisieren"),
            Line::from(" [H] Hilfe ausblenden"),
            Line::from(""),
            Line::from("Überwachung:"),
            Line::from(" • Softwareversion-Check"),
            Line::from(" • Peer-Verbindungssicherheit"),
            Line::from(" • Firewall-Status"),
            Line::from(" • Tor-Verbindungsqualität"),
        ],
        Tab::Explorer => vec![
            Line::from(vec![
                Span::styled("Blockchain Explorer", Style::default().fg(Color::Yellow)),
            ]),
            Line::from(""),
            Line::from("Tastenbelegung:"),
            Line::from(" [B] Block suchen"),
            Line::from(" [T] Transaktion suchen"),
            Line::from(" [A] Adresse suchen"),
            Line::from(" [Enter] Suche ausführen"),
            Line::from(" [Esc] Suche abbrechen"),
            Line::from(""),
            Line::from("Suchoptionen:"),
            Line::from(" • Block: Höhe oder Hash"),
            Line::from(" • Transaktion: TXID"),
            Line::from(" • Adresse: Base58 oder Bech32"),
        ],
    };

    Paragraph::new(content)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title(" Hilfe "))
} 