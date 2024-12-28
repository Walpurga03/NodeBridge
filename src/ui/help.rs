// Für die Hilfe-Ansicht
use super::common::*;
use super::Tab;
use ratatui::{
    widgets::{Wrap, BorderType},
    prelude::Alignment,
};

pub fn create_help(tab: &Tab) -> Paragraph<'static> {
    let content = match tab {
        Tab::Dashboard => vec![
            Line::from(vec![
                Span::styled("Dashboard Übersicht", 
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            ]),
            Line::from(""),
            Line::from("Das Dashboard zeigt den aktuellen Status Ihres Bitcoin Nodes:"),
            Line::from(""),
            Line::from(vec![
                Span::styled("⚡ Netzwerk & Verbindungen", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Netzwerk: main/test/regtest/signet"),
            Line::from(" • Peers: Verbundene Nodes (>8 empfohlen)"),
            Line::from(""),
            Line::from(vec![
                Span::styled("📦 Blockchain Status", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Blöcke: Aktuelle Blockchain-Höhe"),
            Line::from(" • Headers: Bekannte Block-Header"),
            Line::from(" • Difficulty: Mining-Schwierigkeit (je höher, desto schwerer ist es einen Block zu finden)"),
            Line::from("              Aktuelle Difficulty: ~108.52T = 108,522,647,629,298"),
            Line::from(" • Chain Work: Gesamte Mining-Arbeit seit Genesis (Hexadezimal, beginnt mit vielen Nullen)"),
            Line::from("              Format: 0x + 64 Stellen (je mehr Stellen ≠ 0 am Ende, desto mehr Arbeit wurde geleistet)"),
            Line::from(""),
            Line::from(vec![
                Span::styled("🔄 Synchronisation", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Status: Fortschritt in % (grün = synchronisiert)"),
            Line::from(" • IBD: Initial Block Download - Download der kompletten Blockchain"),
            Line::from("        'Aktiv' = Node lädt noch Blöcke herunter"),
            Line::from("        'Abgeschlossen' = Node ist auf aktuellem Stand"),
            Line::from(" • Speicherplatz: Blockchain-Größe auf Festplatte"),
            Line::from(" • Pruned: Reduzierte Blockchain-Größe aktiv?"),
            Line::from(""),
            Line::from(vec![
                Span::styled("💭 Mempool", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Transaktionen: Unbestätigte TXs im lokalen Mempool"),
            Line::from("                  (Kann von anderen Nodes/Websites abweichen,"),
            Line::from("                   da jeder Node seinen eigenen Mempool hat)"),
            Line::from(""),
            Line::from(vec![
                Span::styled("Tastenkombinationen:", 
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • H: Diese Hilfe ein-/ausblenden"),
            Line::from(" • R: Daten aktualisieren"),
            Line::from(" • +/-: Update-Intervall anpassen"),
            Line::from(" • Q: Programm beenden"),
            Line::from(""),
            Line::from(vec![
                Span::styled("Navigation:", 
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • 1-8: Tabs direkt auswählen"),
        ],
        Tab::BlockDetails => vec![
            Line::from(vec![
                Span::styled("Block Details", 
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("📦 Block Information", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Höhe: Position des Blocks in der Blockchain"),
            Line::from(" • Hash: Eindeutige Block-ID (64 Zeichen)"),
            Line::from(" • Zeit: Zeitstempel der Block-Erstellung (UTC)"),
            Line::from(""),
            Line::from(vec![
                Span::styled("🔍 Block Details", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Transaktionen: Vom Miner ausgewählte TXs aus dem Mempool"),
            Line::from("                  (Meist die mit den höchsten Gebühren)"),
            Line::from(" • Größe: Maximale Blockgröße 4MB (Weight Units)"),
            Line::from(" • Gewicht: Tatsächliche Nutzung der 4MB Kapazität"),
            Line::from(""),
            Line::from(vec![
                Span::styled("🔧 Technische Details", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Version: Protokoll-Version des Blocks"),
            Line::from("          Definiert die gültigen Regeln für diesen Block"),
            Line::from(" • Merkle Root: Kryptographische Zusammenfassung aller Transaktionen"),
            Line::from("              Stellt sicher, dass keine Transaktion verändert wurde"),
            Line::from(" • Bits: Zielwert für gültige Block-Hashes"),
            Line::from("        - Ist wie eine 'Messlatte' für die Mining-Schwierigkeit"),
            Line::from("        - Der Block-Hash muss unter diesem Wert liegen"),
            Line::from("        - Wird alle 2 Wochen automatisch angepasst:"),
            Line::from("          • Zu viele Blöcke = Bits wird kleiner (schwieriger)"),
            Line::from("          • Zu wenige Blöcke = Bits wird größer (leichter)"),
            Line::from(" • Nonce: Zufallszahl, die der Miner gefunden hat"),
            Line::from("         Wird so lange verändert, bis ein gültiger Block entsteht"),
            Line::from(""),
            Line::from(vec![
                Span::styled("🔎 Block Suche", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Beim Start auswählen:"),
            Line::from("   1) Aktueller Block"),
            Line::from("   2) Block nach Höhe suchen"),
            Line::from("   3) Block nach Hash suchen"),
            Line::from(""),
            Line::from(vec![
                Span::styled("Tastenkombinationen:", 
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • H: Diese Hilfe ein-/ausblenden"),
            Line::from(" • R: Daten aktualisieren"),
            Line::from(" • Q: Programm beenden"),
        ],
        Tab::Mempool => vec![
            Line::from(vec![
                Span::styled("Mempool Status", 
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("💭 Was ist der Mempool?", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Warteschlange für unbestätigte Bitcoin-Transaktionen"),
            Line::from(" • Jeder Node hat seinen eigenen Mempool"),
            Line::from(" • Miner wählen Transaktionen mit den höchsten Gebühren"),
            Line::from(""),
            Line::from(vec![
                Span::styled("📊 Gebühren-Kategorien", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • No Priority     : Sehr lange Wartezeit (mehrere Tage/Wochen)"),
            Line::from(" • Low Priority    : Längere Wartezeit (~1 Tag)"),
            Line::from(" • Medium Priority : Mittlere Wartezeit (~6 Stunden)"),
            Line::from(" • High Priority   : Schnelle Bestätigung (~20-30 Minuten)"),
            Line::from(""),
            Line::from(" Die sat/vB Werte passen sich dynamisch an die Netzwerkauslastung an."),
            Line::from(" Höhere Auslastung = höhere Gebühren für schnelle Bestätigung."),
            Line::from(""),
            Line::from(" sat/vB = Satoshi pro virtuellem Byte (Transaktionsgröße)"),
            Line::from(" $-Preis basiert auf einer Standard-TX (250 vBytes)"),
            Line::from(""),
            Line::from(vec![
                Span::styled("💾 Speicher-Management", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Aktuell: Momentane Größe des Mempools in MB"),
            Line::from(" • Maximum: Speicherlimit des Mempools (standard: 300 MB)"),
            Line::from("            Wenn voll: Neue TX verdrängt alte TX mit niedrigerer Gebühr"),
            Line::from("            Dies stellt sicher, dass immer die wertvollsten TXs behalten werden"),
            Line::from(""),
            Line::from(vec![
                Span::styled("🔄 Aktualisierung", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Automatisch alle 30 Sekunden"),
            Line::from(" • Manuell mit [R] Taste"),
            Line::from(" • 🔄 zeigt aktive Aktualisierung an"),
            Line::from(""),
            Line::from(vec![
                Span::styled("Hinweise:", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Gebühren werden vom Node geschätzt"),
            Line::from(" • Schätzungen basieren auf:"),
            Line::from("   - Aktuellem Mempool"),
            Line::from("   - Historischen Daten"),
            Line::from("   - Netzwerkaktivität"),
            Line::from(" • Aktualisierung kann einige Sekunden dauern"),
            Line::from(""),
            Line::from(vec![
                Span::styled("Tastenkombinationen:", 
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • H: Diese Hilfe ein-/ausblenden"),
            Line::from(" • R: Daten aktualisieren"),
            Line::from(" • Q: Programm beenden"),
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
            .title(" ℹ️  Hilfe ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD))
            .border_type(BorderType::Double)
            .style(Style::default()
                .bg(Color::Black)))
        .style(Style::default()
            .fg(Color::White)
            .bg(Color::Black))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
} 