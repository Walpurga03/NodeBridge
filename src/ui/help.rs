// Für die Hilfe-Ansicht
use super::common::*;
use super::Tab;
use ratatui::widgets::{Paragraph, BorderType, Wrap};

pub fn create_help(tab: &Tab, _scroll: usize) -> Paragraph<'static> {
    let content = match tab {
        Tab::Dashboard => create_dashboard_help(),
        Tab::BlockDetails => create_block_help(),
        Tab::TxDetails => create_tx_help(),
        Tab::AddressDetails => create_address_help(),
        Tab::Mempool => create_mempool_help(),
        Tab::Network => create_network_help(),
        Tab::PeerList => create_peer_list_help(),
        Tab::Mining => create_mining_help(),
        Tab::Security => create_security_help(),
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
            Span::styled("Block Details - Anatomie eines Bitcoin Blocks", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("📦 Block Identifikation", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Blockhöhe: Position in der Blockchain"),
        Line::from("   → Genesis = Block 0, jeder neue Block +1"),
        Line::from(" • Block Hash: Eindeutige Kennung des Blocks"),
        Line::from("   → Berechnet aus allen Block-Daten"),
        Line::from(" • Zeitstempel: Erstellungszeit des Blocks"),
        Line::from("   → Muss zwischen Medianzeit der letzten Blöcke und 2h in Zukunft liegen"),
        Line::from(""),
        Line::from(vec![
            Span::styled("📊 Block Größen", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Transaktionen: Anzahl der enthaltenen TXs"),
        Line::from("   → Erste TX ist immer die Mining-Belohnung"),
        Line::from(" • Größe: Speicherplatz des Blocks in Bytes"),
        Line::from("   → 1 Byte = 8 Bits, max. 4.000.000 Bytes pro Block"),
        Line::from(" • Gewicht: Neue Messung seit SegWit-Update"),
        Line::from("   → Maximal 4.000.000 Gewichtseinheiten pro Block"),
        Line::from(""),
        Line::from(vec![
            Span::styled("⛏️ Mining Informationen", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Version: Protokoll-Version des Blocks"),
        Line::from("   → Zeigt unterstützte Bitcoin-Funktionen"),
        Line::from(" • Merkle Root: Prüfsumme aller Transaktionen"),
        Line::from("   → Ermöglicht schnelles Verifizieren der TXs"),
        Line::from(" • Bits: Aktuelle Mining-Schwierigkeit"),
        Line::from("   → Je kleiner die Zahl, desto schwieriger"),
        Line::from(" • Nonce: Zufallszahl für Mining"),
        Line::from("   → Wird verändert bis gültiger Block gefunden"),
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
            Span::styled("Dashboard - Ihr Bitcoin Node auf einen Blick", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(""),
        Line::from("Willkommen! Hier sehen Sie den Status Ihres Bitcoin Nodes:"),
        Line::from(""),
        Line::from(vec![
            Span::styled("⚡ Netzwerk", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Mainnet: Das produktive Bitcoin-Netzwerk"),
        Line::from(" • Testnet: Testnetzwerk für Entwicklung (kostenlose Testnet-BTC)"),
        Line::from(" • Peers: Verbundene Bitcoin Nodes im P2P-Netzwerk"),
        Line::from("   → Mindestens 8 Peers für optimale Dezentralisierung"),
        Line::from("   → Mehr Peers = bessere Netzwerkresilienz"),
        Line::from(""),
        Line::from(vec![
            Span::styled("📦 Blockchain", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Blöcke: Anzahl validierter Blöcke in der Blockchain"),
        Line::from("   → Jeder Block enthält mehrere Transaktionen"),
        Line::from(" • Headers: Block-Header der bekannten Blöcke"),
        Line::from("   → Sollte identisch mit der Blockhöhe sein"),
        Line::from(" • Difficulty: Mining-Schwierigkeit des Netzwerks"),
        Line::from("   → Automatische Anpassung alle 2016 Blöcke"),
        Line::from(" • Chain Work: Kumulierter Proof-of-Work der Chain"),
        Line::from("   → Maß für die Sicherheit der Blockchain"),
        Line::from(""),
        Line::from(vec![
            Span::styled("🔄 Status", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Fortschritt: Initial Block Download (IBD) Status"),
        Line::from("   → 100% = Node ist vollständig synchronisiert"),
        Line::from(" • Speicher: Blockchain-Größe auf der Festplatte"),
        Line::from("   → Full Node benötigt aktuell etwa 500+ GB"),
        Line::from(" • Pruned: Node-Modus für reduzierten Speicherbedarf"),
        Line::from("   → 'Ja' = Nur neuere Blöcke werden vorgehalten"),
        Line::from("   → 'Nein' = Vollständige Blockchain wird gespeichert"),
        Line::from(""),
        Line::from(vec![
            Span::styled("💭 Mempool", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Unbestätigte Transaktionen im lokalen Mempool"),
        Line::from(" • Größerer Mempool = höhere Netzwerk-Auslastung"),
        Line::from(" • Details zu Gebühren (sat/vB) im Mempool-Tab"),
        Line::from(""),
        Line::from(vec![
            Span::styled("⌨️ Steuerung", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • H: Diese Hilfe zeigen/verstecken"),
        Line::from(" • Q: Programm beenden"),
        Line::from(" • 1-9: Schnell zwischen Tabs wechseln"),
    ]
}

fn create_tx_help() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            Span::styled("Transaktionsdetails (TX)", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("💰 Inputs & Outputs (UTXO-Modell)", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Inputs (Herkunft)                    • Outputs (Ziel)"),
        Line::from("   → Verweisen auf frühere UTXOs          → Neue UTXOs (Unspent Transaction Outputs)"),
        Line::from("   → Format: Betrag + TXID               → Format: Betrag + Empfänger-Adresse"),
        Line::from("   → Müssen komplett verbraucht werden   → TXID:Index identifiziert jeden Output"),
        Line::from(""),
        Line::from(" • Beispiel einer Bitcoin-Transaktion:"),
        Line::from("   ┌──────────────────────────────────────────────────────────┐"),
        Line::from("   │ Input:    2.0 BTC (von TXID abc123)                      │"),
        Line::from("   │ Output 1: 1.2 BTC an Alice     (TXID def456:0)          │"),
        Line::from("   │ Output 2: 0.7 BTC zurück       (TXID def456:1)          │"),
        Line::from("   │ Gebühr:   0.1 BTC (Differenz Input-Output)              │"),
        Line::from("   └──────────────────────────────────────────────────────────┘"),
        Line::from(""),
        Line::from(vec![
            Span::styled("📏 Größenangaben", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Bytes (physikalisch)                 • Weight Units (WU)"),
        Line::from("   → Tatsächliche Größe der TX           → Interne SegWit-Berechnung"),
        Line::from("   → Alle TX-Daten inkl. Signaturen     → 1 vByte = 4 WU"),
        Line::from(""),
        Line::from(" • vBytes (virtuell)"),
        Line::from("   → Basis für Gebührenberechnung (sat/vB)"),
        Line::from("   → Normale Daten: 4 WU = 1 vByte"),
        Line::from("   → Signatur-Daten: 1 WU = 0.25 vByte"),
        Line::from(""),
        Line::from(vec![
            Span::styled("🕒 Status", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Bestätigung                          • Block"),
        Line::from("   → Bestätigt = In einem Block           → Blockhash = Block-ID"),
        Line::from("   → Unbestätigt = Im Mempool            → Anzahl Bestätigungen seit Aufnahme"),
    ]
}

fn create_address_help() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            Span::styled("Adressdetails", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("📋 Allgemeine Information", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Adresstyp: P2PKH, P2SH, P2WPKH, P2WSH oder P2TR"),
        Line::from(" • Erste Aktivität: Zeitpunkt der ersten Transaktion"),
        Line::from(" • Letzte Aktivität: Zeitpunkt der letzten Transaktion"),
        Line::from(""),
        Line::from(vec![
            Span::styled("💰 Finanzen", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Aktueller Kontostand in BTC"),
        Line::from(" • Gesamtbetrag empfangen"),
        Line::from(" • Gesamtbetrag gesendet"),
        Line::from(""),
        Line::from(vec![
            Span::styled("📊 Statistiken", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Anzahl Transaktionen"),
        Line::from(" • Empfangene UTXOs"),
        Line::from(" • Ausgegebene UTXOs"),
        Line::from(" • Unausgegebene UTXOs"),
        Line::from(""),
        Line::from(vec![
            Span::styled("⚡ Mempool", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Unbestätigte Transaktionen"),
        Line::from(" • Eingehende Beträge"),
        Line::from(" • Ausgehende Beträge"),
    ]
}

fn create_mempool_help() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            Span::styled("Mempool Übersicht", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("📊 Statistiken", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Anzahl Transaktionen: Aktuell unbestätigte TXs"),
        Line::from(" • Gesamtgröße: Speicherbedarf in MB"),
        Line::from(" • Gesamtgebühren: Summe aller TX-Gebühren"),
        Line::from(""),
        Line::from(vec![
            Span::styled("💰 Gebührenkategorien", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Niedrig: 1-5 sat/vB"),
        Line::from(" • Mittel: 6-20 sat/vB"),
        Line::from(" • Hoch: >20 sat/vB"),
        Line::from(""),
        Line::from(vec![
            Span::styled("⏳ Schätzungen", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Nächster Block: Wahrscheinliche TXs"),
        Line::from(" • Wartezeit: Geschätzt pro Kategorie"),
    ]
}

fn create_network_help() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            Span::styled("Netzwerk Übersicht", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("🌐 Verbindungen", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Eingehend: Verbindungen zu Ihrem Node"),
        Line::from(" • Ausgehend: Verbindungen zu anderen Nodes"),
        Line::from(" • Gesamt: Summe aller Verbindungen"),
        Line::from(""),
        Line::from(vec![
            Span::styled("📡 Datenverkehr", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Gesendet: Ausgehende Daten"),
        Line::from(" • Empfangen: Eingehende Daten"),
        Line::from(" • Bandbreite: Aktuelle Nutzung"),
        Line::from(""),
        Line::from(vec![
            Span::styled("🔒 Version", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Protokoll: Bitcoin P2P Version"),
        Line::from(" • User Agent: Client-Identifikation"),
        Line::from(" • Services: Angebotene Dienste"),
    ]
}

fn create_peer_list_help() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            Span::styled("Peer Liste", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("👥 Verbindungen", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • IP/Port: Netzwerkadresse des Peers"),
        Line::from(" • Version: Bitcoin Core Version"),
        Line::from(" • Dienste: Angebotene Services"),
        Line::from(""),
        Line::from(vec![
            Span::styled("📊 Statistiken", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Ping: Verbindungsqualität in ms"),
        Line::from(" • Gesendet: Ausgehende Bytes"),
        Line::from(" • Empfangen: Eingehende Bytes"),
        Line::from(""),
        Line::from(vec![
            Span::styled("⏱️ Zeitangaben", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Verbunden seit: Dauer der Verbindung"),
        Line::from(" • Letzter Block: Zeitpunkt des letzten Blocks"),
        Line::from(" • Synchronisation: Fortschritt in %"),
    ]
}

fn create_mining_help() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            Span::styled("Mining Information", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("⛏️ Mining Status", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Schwierigkeit: Aktuelle Mining-Difficulty"),
        Line::from(" • Hashrate: Geschätzte Netzwerk-Hashrate"),
        Line::from(" • Nächste Anpassung: Blocks/Zeit bis Difficulty-Change"),
        Line::from(""),
        Line::from(vec![
            Span::styled("📈 Block Statistiken", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Letzte Anpassung: Änderung in %"),
        Line::from(" • Durchschnittliche Blockzeit"),
        Line::from(" • Blocks seit letzter Anpassung"),
        Line::from(""),
        Line::from(vec![
            Span::styled("💰 Belohnungen", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Block Subsidy: Aktuelle Block-Belohnung"),
        Line::from(" • Nächste Halbierung: Blocks/Zeit"),
        Line::from(" • Durchschnittliche Gebühren/Block"),
    ]
}

fn create_security_help() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            Span::styled("Sicherheit", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("🔒 Node Sicherheit", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Verbindungen: Verschlüsselt/Unverschlüsselt"),
        Line::from(" • Authentifizierung: RPC Zugriffskontrolle"),
        Line::from(" • Firewall: Port-Freigaben & Regeln"),
        Line::from(""),
        Line::from(vec![
            Span::styled("🛡️ Blockchain", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Verifizierung: Signatur-Checks"),
        Line::from(" • Konsens: Aktuelle Regeln"),
        Line::from(" • Chain Work: Proof-of-Work Sicherheit"),
        Line::from(""),
        Line::from(vec![
            Span::styled("⚠️ Warnungen", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(" • Version: Sicherheitsupdates verfügbar"),
        Line::from(" • Netzwerk: Verbindungsprobleme"),
        Line::from(" • System: Ressourcenauslastung"),
    ]
} 