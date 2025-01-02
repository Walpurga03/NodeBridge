// Für die Hilfe-Ansicht
use super::common::*;
use super::Tab;
use ratatui::widgets::{Paragraph, BorderType, Wrap};

pub fn create_help(tab: &Tab, _scroll: usize) -> Paragraph<'static> {
    let content = match tab {
        Tab::Dashboard => create_dashboard_help(),
        Tab::BlockDetails => create_block_help(),
        Tab::TxDetails => vec![
            Line::from(vec![
                Span::styled("Transaktionsdetails", 
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("💰 Inputs & Outputs", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Inputs zeigen die Herkunft der Bitcoins:"),
            Line::from("   - Format: X.XXX BTC (TX: [txid])"),
            Line::from("   - Referenzieren frühere Transaktionen"),
            Line::from("   - Summe der Inputs = Summe der Outputs + Transaktionsgebühr"),
            Line::from(""),
            Line::from(" • Outputs (UTXOs):"),
            Line::from("   - Format: X.XXX BTC (TX: [aktuelle_txid]:[index])"),
            Line::from("   - Alle Outputs einer TX teilen sich die gleiche TXID"),
            Line::from("   - Der Index unterscheidet die verschiedenen Outputs"),
            Line::from("   - Jeder Output geht an eine andere Bitcoin-Adresse"),
            Line::from("   - Beispiel für eine TX mit ID 'def456':"),
            Line::from("     Input:  1.5 BTC von 1ABC... (TX: abc123...)"),
            Line::from("     Output: 1.0 BTC an bc1GHI... (TX: def456:0)"),
            Line::from("     Output: 0.49 BTC an 3JKL... (TX: def456:1)"),
            Line::from("     Gebühr: 0.01 BTC an den Miner"),
            Line::from(""),
            Line::from(" • Wichtig zu verstehen:"),
            Line::from("   - Inputs verweisen auf frühere Transaktionen"),
            Line::from("   - Outputs sind Teil der aktuellen Transaktion"),
            Line::from("   - Outputs werden später als Inputs verwendet"),
            Line::from(""),
            Line::from(vec![
                Span::styled("📊 Technische Details", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Größe in Bytes:"),
            Line::from("   - Tatsächlicher Speicherplatz auf der Blockchain"),
            Line::from("   - Enthält alle Transaktionsdaten (Inputs, Outputs, Signaturen)"),
            Line::from("   - 1 Byte = 8 Bit (kleinste Speichereinheit)"),
            Line::from(""),
            Line::from(" • Virtuelle Größe (vBytes):"),
            Line::from("   - Spezielle Einheit für SegWit-Transaktionen"),
            Line::from("   - Basis für die Gebührenberechnung"),
            Line::from("   - Reduziert die effektive Größe von Witness-Daten"),
            Line::from("   - 1 vByte = 4 Weight Units (WU)"),
            Line::from(""),
            Line::from(" • Gewicht (Weight Units, WU):"),
            Line::from("   - Eingeführt mit SegWit-Update"),
            Line::from("   - Maximales Block-Limit: 4.000.000 WU"),
            Line::from("   - Normale Daten: 4 WU pro Byte"),
            Line::from("   - Witness-Daten: 1 WU pro Byte"),
            Line::from("   - Ermöglicht mehr Transaktionen pro Block"),
            Line::from(""),
            Line::from(" • Beispielrechnung:"),
            Line::from("   - TX mit 200 Byte normalen Daten = 800 WU"),
            Line::from("   - Plus 100 Byte Witness-Daten = 100 WU"),
            Line::from("   - Gesamt: 900 WU = 225 vBytes"),
            Line::from(""),
            Line::from(vec![
                Span::styled("🕒 Zeit & Block", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]),
            Line::from(" • Zeit: Bestätigungszeitpunkt"),
            Line::from("   - Wann die TX in einen Block aufgenommen wurde"),
            Line::from("   - Bei unbestätigten TXs: 'Noch nicht bestätigt'"),
            Line::from(""),
            Line::from(" • Block Hash: Eindeutige Block-ID"),
            Line::from("   - Zeigt den Block, der die TX enthält"),
            Line::from("   - Bei unbestätigten TXs: leer"),
            Line::from(""),
        ],
        _ => create_default_help(),
    };

    Paragraph::new(content)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Hilfe")
            .border_type(BorderType::Rounded))
        .wrap(Wrap { trim: true })
}

// Hilfsfunktion für Block-Details Hilfe
fn create_block_help() -> Vec<Line<'static>> {
    vec![
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
        Line::from(" • Transaktionen: Vom Miner ausgewählte TXs"),
        Line::from(" • Größe: Tatsächliche Größe des Blocks in Bytes"),
        Line::from(" • Gewicht: Alternative Messung der Blockgröße"),
        Line::from(""),
        Line::from(vec![
            Span::styled("🔧 Technische Details", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Version: Protokoll-Version des Blocks"),
        Line::from(" • Merkle Root: Hash aller Transaktionen"),
        Line::from(" • Bits: Schwierigkeitsgrad für Mining"),
        Line::from(" • Nonce: Vom Miner gefundene Lösung"),
    ]
}

// Standard-Hilfe für nicht spezifizierte Tabs
fn create_default_help() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            Span::styled("Hilfe", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(""),
        Line::from("Wählen Sie einen Tab aus, um spezifische Hilfe anzuzeigen."),
        Line::from(""),
        Line::from("Tastenkombinationen:"),
        Line::from(" • H: Diese Hilfe ein-/ausblenden"),
        Line::from(" • Q: Programm beenden"),
        Line::from(" • C: Kopier-Modus aktivieren/deaktivieren"),
        Line::from(" • ESC: Kopier-Modus beenden"),
        Line::from(""),
        Line::from("Kopier-Modus:"),
        Line::from(" • Aktivieren Sie den Modus mit 'C'"),
        Line::from(" • Wichtige Werte werden hervorgehoben"),
        Line::from(" • Kopieren Sie mit der Maus oder Strg+Shift+C"),
        Line::from(" • Beenden Sie den Modus mit ESC"),
    ]
}

#[allow(dead_code)]
/// Zeigt die Hilfe für das Wallet-Modul an
pub fn wallet_help() {
    println!("Wallet-Integration hilft Ihnen, verschiedene Wallets zu verwalten und Transaktionen zu signieren.");
}

#[allow(dead_code)]
/// Zeigt die Hilfe für den Export-Modul an
pub fn export_help() {
    println!("Export-Funktionen ermöglichen das Exportieren von Daten in verschiedenen Formaten wie CSV und JSON.");
}

// Für das Dashboard-Tab
fn create_dashboard_help() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            Span::styled("Dashboard Übersicht", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(""),
        Line::from("Das Dashboard zeigt den aktuellen Status Ihres Bitcoin Nodes an und bietet folgende Informationen:"),
        Line::from(""),
        Line::from(" • Netzwerk: Zeigt das aktuelle Netzwerk an, in dem Ihr Node läuft (z.B. main, test, regtest, signet)."),
        Line::from(" • Peers: Anzahl der verbundenen Nodes. Mehr als 8 Verbindungen werden empfohlen für eine stabile Netzwerkverbindung."),
        Line::from(""),
        Line::from(" • Blöcke: Die aktuelle Höhe der Blockchain, d.h. die Anzahl der Blöcke, die Ihr Node kennt."),
        Line::from(" • Headers: Anzahl der bekannten Block-Header. Sollte gleich oder größer als die Anzahl der Blöcke sein."),
        Line::from(" • Difficulty: Die aktuelle Mining-Schwierigkeit. Je höher der Wert, desto schwieriger ist es, einen neuen Block zu finden."),
        Line::from(" • Chain Work: Die gesamte Arbeit, die seit dem Genesis-Block geleistet wurde. Ein Maß für die Sicherheit der Blockchain."),
        Line::from(""),
        Line::from(" • Status: Der Fortschritt der Blockchain-Synchronisation in Prozent. Grün bedeutet vollständig synchronisiert."),
        Line::from(" • IBD: Initial Block Download. Zeigt an, ob der Node noch Blöcke herunterlädt oder bereits auf dem neuesten Stand ist."),
        Line::from(" • Speicherplatz: Der Speicherplatz, den die Blockchain auf Ihrer Festplatte belegt."),
        Line::from(" • Pruned: Gibt an, ob die Blockchain-Größe reduziert wurde, um Speicherplatz zu sparen."),
        Line::from(""),
        Line::from(" • Mempool: Anzahl der unbestätigten Transaktionen im lokalen Mempool. Kann von anderen Nodes abweichen."),
        // Netzwerk Sektion
        Line::from(vec![
            Span::styled("⚡ Netzwerk & Verbindungen", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Netzwerk: main/test/regtest/signet"),
        Line::from(" • Peers: Verbundene Nodes (>8 empfohlen)"),
        Line::from(""),
        // Blockchain Status
        Line::from(vec![
            Span::styled("📦 Blockchain Status", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Blöcke: Aktuelle Blockchain-Höhe"),
        Line::from(" • Headers: Bekannte Block-Header"),
        Line::from(" • Difficulty: Mining-Schwierigkeit: 108,522,647,020,298"),
        Line::from(" • Chain Work: Gesamte Mining-Arbeit: 0000000000000000000000000000000000000000000000000000000000000000"),
        Line::from(""),
        // Synchronisation
        Line::from(vec![
            Span::styled("🔄 Synchronisation", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Status: Fortschritt in % (grün = synchronisiert)"),
        Line::from(" • IBD: Initial Block Download - Download der kompletten Blockchain"),
        Line::from("   'Aktiv' = Node lädt noch Blöcke herunter"),
        Line::from("   'Abgeschlossen' = Node ist auf aktuellem Stand"),
        Line::from(" • Speicherplatz: Blockchain-Größe auf Festplatte"),
        Line::from(" • Pruned: Reduzierte Blockchain-Größe aktiv?"),
        Line::from(""),
        // Mempool
        Line::from(vec![
            Span::styled("💭 Mempool", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Transaktionen: Unbestätigte TXs im lokalen Mempool"),
        Line::from(" • Kann von anderen Nodes/Websites abweichen,"),
        Line::from("   da jeder Node seinen eigenen Mempool hat"),
        Line::from(""),
        // Tastenkombinationen
        Line::from(vec![
            Span::styled("Tastenkombinationen:", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • H: Diese Hilfe ein-/ausblenden"),
        Line::from(" • Q: Programm beenden"),
        Line::from(" • C: Kopier-Modus aktivieren/deaktivieren"),
        Line::from(" • ESC: Kopier-Modus beenden"),
        Line::from(""),
        // Navigation
        Line::from(vec![
            Span::styled("Navigation:", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • 1-9: Tabs direkt auswählen"),
    ]
} 