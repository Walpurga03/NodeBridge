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
                Span::styled("Netzwerk & Peers", 
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("⚡ Netzwerk Status", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Netzwerk: main = Mainnet, test = Testnet"),
            Line::from(" • Synchronisation: Fortschritt des Block-Downloads"),
            Line::from(""),
            Line::from(vec![
                Span::styled("🔌 Verbindungen", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Gesamt: Anzahl aller Verbindungen (mindestens 8 empfohlen)"),
            Line::from(" • Eingehend: Andere Nodes verbinden sich zu uns"),
            Line::from(" • Ausgehend: Wir verbinden uns zu anderen Nodes"),
            Line::from(" • Tor: Verbindungen über das Tor-Netzwerk (Privatsphäre)"),
            Line::from(""),
            Line::from(vec![
                Span::styled("📊 Latenz-Werte", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(vec![
                Span::raw(" • "),
                Span::styled("Grün", Style::default().fg(Color::Green)),
                Span::raw(" < 100ms = Sehr gute Verbindung"),
            ]),
            Line::from(vec![
                Span::raw(" • "),
                Span::styled("Gelb", Style::default().fg(Color::Yellow)),
                Span::raw(" 100-500ms = Normale Verbindung"),
            ]),
            Line::from(vec![
                Span::raw(" • "),
                Span::styled("Rot", Style::default().fg(Color::Red)),
                Span::raw(" > 500ms = Langsame Verbindung"),
            ]),
            Line::from(""),
            Line::from(" Die Latenz zeigt die Reaktionszeit der Verbindungen:"),
            Line::from(" • Minimum = Schnellste Verbindung"),
            Line::from(" • Maximum = Langsamste Verbindung"),
            Line::from(" • Durchschnitt = Mittlere Geschwindigkeit aller Verbindungen"),
            Line::from(""),
            Line::from(vec![
                Span::styled("Tastenkombinationen:", 
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • H: Diese Hilfe ein-/ausblenden"),
            Line::from(" • R: Daten aktualisieren"),
            Line::from(" • Q: Programm beenden"),
        ],
        Tab::PeerList => vec![
            Line::from(vec![
                Span::styled("Peer Liste", 
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("📋 Spalten-Erklärung", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Adresse: IP oder .onion Adresse des Peers"),
            Line::from("   - 🧅 = Tor-Verbindung (anonym)"),
            Line::from("   - IP = Direkte Verbindung"),
            Line::from(""),
            Line::from(" • Version: Bitcoin Client und Version des Peers"),
            Line::from("   z.B. /Satoshi:25.0.0/ = Bitcoin Core v25.0.0"),
            Line::from(""),
            Line::from(" • Latenz: Reaktionszeit der Verbindung"),
            Line::from(vec![
                Span::raw("   - "),
                Span::styled("Grün", Style::default().fg(Color::Green)),
                Span::raw(" < 100ms: Sehr schnell"),
            ]),
            Line::from(vec![
                Span::raw("   - "),
                Span::styled("Gelb", Style::default().fg(Color::Yellow)),
                Span::raw(" 100-500ms: Normal"),
            ]),
            Line::from(vec![
                Span::raw("   - "),
                Span::styled("Rot", Style::default().fg(Color::Red)),
                Span::raw(" > 500ms: Langsam"),
            ]),
            Line::from(""),
            Line::from(" • Verbunden: Zeit seit Verbindungsaufbau"),
            Line::from("   - d = Tage"),
            Line::from("   - h = Stunden"),
            Line::from("   - m = Minuten"),
            Line::from(""),
            Line::from(" • Senden/Empfangen: Datenübertragung in MB"),
            Line::from("   - Links: Gesendete Daten"),
            Line::from("   - Rechts: Empfangene Daten"),
            Line::from(""),
            Line::from(" • Typ: Art der Verbindung"),
            Line::from("   - Eingehend: Peer hat sich mit uns verbunden"),
            Line::from("   - Ausgehend: Wir haben uns mit Peer verbunden"),
            Line::from(""),
            Line::from(vec![
                Span::styled("ℹ️  Hinweise", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Liste ist nach Latenz sortiert (schnellste zuerst)"),
            Line::from(" • Ausgewogene Mischung aus Ein-/Ausgehend ist optimal"),
            Line::from(" • Mehrere Tor-Verbindungen erhöhen die Privatsphäre"),
            Line::from(""),
            Line::from(vec![
                Span::styled("Tastenkombinationen:", 
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • H: Diese Hilfe ein-/ausblenden"),
            Line::from(" • R: Daten aktualisieren"),
            Line::from(" • Q: Programm beenden"),
        ],
        Tab::Mining => vec![
            Line::from(vec![
                Span::styled("Mining Status", 
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("📊 Difficulty", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Schwierigkeitsgrad für das Mining"),
            Line::from(" • Wissenschaftliche Notation (z.B. 5.3e13)"),
            Line::from(" • Dezimaldarstellung zum Vergleich"),
            Line::from(""),
            Line::from(vec![
                Span::styled("🔄 Difficulty-Anpassung", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Findet alle 2016 Blöcke statt (~2 Wochen)"),
            Line::from(" • Ziel: 1 Block alle 10 Minuten"),
            Line::from(" • Grün = wird leichter"),
            Line::from(" • Rot = wird schwerer"),
            Line::from(""),
            Line::from(vec![
                Span::styled("⚡ Hashrate", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Rechenleistung des gesamten Netzwerks"),
            Line::from(" • Einheiten:"),
            Line::from("   - EH/s = Exa-Hashes pro Sekunde (10¹⁸)"),
            Line::from("   - PH/s = Peta-Hashes pro Sekunde (10¹⁵)"),
            Line::from("   - TH/s = Tera-Hashes pro Sekunde (10¹²)"),
            Line::from(""),
            Line::from(vec![
                Span::styled("Tastenkombinationen:", 
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • H: Diese Hilfe ein-/ausblenden"),
            Line::from(" • R: Daten aktualisieren"),
            Line::from(" • Q: Programm beenden"),
        ],
        Tab::Security => vec![
            Line::from(vec![
                Span::styled("Sicherheitsstatus", 
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("📦 Software", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Version: Aktuelle Bitcoin Core Version"),
            Line::from(vec![
                Span::raw(" • "),
                Span::styled("Grün", Style::default().fg(Color::Green)),
                Span::raw(" = Aktuell, "),
                Span::styled("Rot", Style::default().fg(Color::Red)),
                Span::raw(" = Update verfügbar"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("🔌 Netzwerk", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Verbindungen: Verhältnis Tor zu direkten Verbindungen"),
            Line::from(vec![
                Span::raw(" • "),
                Span::styled("Grün", Style::default().fg(Color::Green)),
                Span::raw(" = Mehr Tor als direkte Verbindungen (besser für Privatsphäre)"),
            ]),
            Line::from(" • Firewall: Schutz vor unerwünschten Verbindungen"),
            Line::from(" • Tor: Anonymes Routing für mehr Privatsphäre"),
            Line::from(""),
            Line::from(vec![
                Span::styled("🔐 Zugriffskontrolle", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • RPC Zugriff: API-Zugriffsbeschränkungen"),
            Line::from("   - IP-Beschränkung"),
            Line::from("   - SSL/TLS Verschlüsselung"),
            Line::from("   - Benutzer-Authentifizierung"),
            Line::from("   - Befehlseinschränkungen"),
            Line::from(" • Wallet: Verschlüsselung der Wallet-Datei"),
            Line::from(" • Festplatte: System-Verschlüsselung"),
            Line::from(""),
            Line::from(vec![
                Span::styled("⏰ System", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Uptime: Laufzeit seit letztem Neustart"),
            Line::from(" • Backup: Status der Wallet-Sicherung"),
            Line::from(vec![
                Span::raw(" • "),
                Span::styled("Grün", Style::default().fg(Color::Green)),
                Span::raw(" = < 7 Tage, "),
                Span::styled("Gelb", Style::default().fg(Color::Yellow)),
                Span::raw(" = > 7 Tage, "),
                Span::styled("Rot", Style::default().fg(Color::Red)),
                Span::raw(" = Kein Backup"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Tastenkombinationen:", 
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • H: Diese Hilfe ein-/ausblenden"),
            Line::from(" • R: Daten aktualisieren"),
            Line::from(" • Q: Programm beenden"),
        ],
        Tab::TxDetails => vec![
            Line::from(vec![
                Span::styled("Transaction Details", 
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("📋 Informationen", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • TXID: Eindeutige Transaktions-ID (64 Zeichen Hex)"),
            Line::from(""),
            Line::from(vec![
                Span::styled("💰 Inputs & Outputs", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Inputs: Vorherige Transaktionen, die als Quelle dienen"),
            Line::from("   - Zeigt die TXIDs der Quell-Transaktionen"),
            Line::from("   - Jeder Input verweist auf einen früheren Output"),
            Line::from(""),
            Line::from(" • Outputs: Empfänger und gesendete Beträge"),
            Line::from("   - Zeigt Betrag in BTC und Empfänger-Adresse"),
            Line::from("   - Format: [Betrag] BTC → [Adresse]"),
            Line::from(""),
            Line::from(vec![
                Span::styled("🔧 Technische Details", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Größe: Tatsächliche Größe in Bytes"),
            Line::from("   - Wie viel Speicherplatz die Transaktion benötigt"),
            Line::from("   - Je größer, desto höhere Transaktionsgebühren"),
            Line::from("   - Typische Größen: 200-500 Bytes für einfache TX"),
            Line::from(""),
            Line::from(" • Gewicht (Weight Units / WU):"),
            Line::from("   - Ein Maß für die 'Schwere' der Transaktion"),
            Line::from("   - Neueres System für fairere Gebührenberechnung"),
            Line::from("   - SegWit-Transaktionen sparen Gebühren"),
            Line::from("   - Typische Werte: 800-2000 WU für normale TX"),
            Line::from(""),
            Line::from(" • Zeit: Wann wurde die Transaktion verarbeitet?"),
            Line::from("   - Zeitpunkt der ersten Bestätigung im Netzwerk"),
            Line::from("   - Wichtig für die chronologische Reihenfolge"),
            Line::from(""),
            Line::from(" • Block: In welchem Block ist die TX gespeichert?"),
            Line::from("   - Jeder Block hat eine eindeutige Block-Hash"),
            Line::from("   - Zeigt, wann die TX endgültig bestätigt wurde"),
            Line::from("   - Je mehr Blöcke danach, desto sicherer"),
            Line::from(""),
            Line::from(vec![
                Span::styled("Tastenkombinationen:", 
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • H: Diese Hilfe ein-/ausblenden"),
            Line::from(" • R: Daten aktualisieren"),
            Line::from(" • Q: Programm beenden"),
        ],
        Tab::AddressDetails => vec![
            Line::from(vec![
                Span::styled("Address Details", 
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("📋 Informationen", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Adresse: Bitcoin-Adresse oder Public Key"),
            Line::from(""),
            Line::from(vec![
                Span::styled("🔍 Adresstypen", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Native SegWit (bc1...)"),
            Line::from("   - Neuester Standard"),
            Line::from("   - Niedrigste Gebühren"),
            Line::from("   - Beginnt mit 'bc1'"),
            Line::from(""),
            Line::from(" • Nested SegWit (3...)"),
            Line::from("   - Kompatibilitätsformat"),
            Line::from("   - Mittlere Gebühren"),
            Line::from("   - Beginnt mit '3'"),
            Line::from(""),
            Line::from(" • Legacy (1...)"),
            Line::from("   - Ursprüngliches Format"),
            Line::from("   - Höchste Gebühren"),
            Line::from("   - Beginnt mit '1'"),
            Line::from(""),
            Line::from(" • Public Key (Hex)"),
            Line::from("   - 130 Zeichen hexadezimal"),
            Line::from("   - Wird für spezielle Anwendungen verwendet"),
            Line::from(""),
            Line::from(vec![
                Span::styled("💰 Details", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Balance: Aktuelles Guthaben in BTC"),
            Line::from(" • Transaktionen: Anzahl aller Transaktionen"),
            Line::from(" • Script: Technische Details zum Adresstyp"),
            Line::from(""),
            Line::from(vec![
                Span::styled("Tastenkombinationen:", 
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • H: Diese Hilfe ein-/ausblenden"),
            Line::from(" • R: Daten aktualisieren"),
            Line::from(" • Q: Programm beenden"),
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