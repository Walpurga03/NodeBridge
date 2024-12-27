// Für die Hilfe-Ansicht
use super::common::*;
use super::Tab;
use ratatui::widgets::Wrap;

pub fn create_help(tab: &Tab) -> Paragraph<'static> {
    let content = match tab {
        Tab::Dashboard => vec![
            Line::from(vec![
                Span::styled("Dashboard Übersicht", 
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            ]),
            Line::from(""),
            Line::from("Das Dashboard zeigt die wichtigsten Informationen über Ihren Bitcoin Node:"),
            Line::from(""),
            Line::from(vec![
                Span::styled("⚡ Netzwerk & Verbindungen", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Netzwerk: main/test/regtest - Zeigt das aktuelle Bitcoin-Netzwerk"),
            Line::from(" • Peers: Anzahl der verbundenen Nodes (grün wenn >8 Verbindungen)"),
            Line::from(""),
            Line::from(vec![
                Span::styled("📦 Blockchain Status", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Blöcke: Aktuelle Blockchain-Höhe"),
            Line::from(" • Headers: Anzahl der bekannten Block-Header"),
            Line::from(" • Difficulty: Aktuelle Mining-Schwierigkeit"),
            Line::from(" • Chain Work: Kumulative Arbeit in der Blockchain (in Hex)"),
            Line::from(""),
            Line::from(vec![
                Span::styled("🔄 Synchronisation", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Status: Synchronisationsfortschritt in Prozent"),
            Line::from(" • IBD: Initial Block Download Status"),
            Line::from(" • Speicherplatz: Größe der Blockchain auf der Festplatte"),
            Line::from(" • Pruned: Zeigt an, ob der Node im Pruning-Modus läuft"),
            Line::from(""),
            Line::from(vec![
                Span::styled("💭 Mempool", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Transaktionen: Anzahl der unbestätigten Transaktionen"),
            Line::from(""),
            Line::from(vec![
                Span::styled("Tastenkombinationen:", 
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • H: Diese Hilfe anzeigen/ausblenden"),
            Line::from(" • 1-8: Direktauswahl der Tabs"),
            Line::from(" • Q: Programm beenden"),
            Line::from(""),
            Line::from(vec![
                Span::styled("Tab-Auswahl:", 
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" 1: Dashboard"),
            Line::from(" 2: Block Details"),
            Line::from(" 3: Mempool"),
            Line::from(" 4: Netzwerk"),
            Line::from(" 5: Peer Liste"),
            Line::from(" 6: Mining"),
            Line::from(" 7: Security"),
            Line::from(" 8: Explorer"),
        ],
        Tab::BlockDetails => vec![
            Line::from(vec![
                Span::styled("Block Details", Style::default().fg(Color::Yellow)),
            ]),
            Line::from(""),
            Line::from(" • Aktuelle Blockhöhe"),
            Line::from(" • Block Hash"),
            Line::from(" • Zeitstempel"),
            Line::from(""),
            Line::from("Tastenkombinationen:"),
            Line::from(" • H: Hilfe anzeigen/ausblenden"),
            Line::from(" • 1-8: Direktauswahl der Tabs"),
            Line::from(" • Q: Programm beenden"),
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
            .title(" Hilfe ")
            .borders(Borders::ALL))
        .wrap(Wrap { trim: true })
} 