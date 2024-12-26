# Bitcoin Node Terminal UI

## Projektbeschreibung
Eine Terminal-basierte Benutzeroberfläche für die Interaktion mit einem Bitcoin Node, die es ermöglicht, wichtige Node-Informationen zu überwachen und grundlegende Operationen auszuführen.

## Hauptziele
1. Einfache Überwachung des Bitcoin Node Status
2. Anzeige wichtiger Blockchain-Informationen
3. Wallet-Management Funktionen
4. Benutzerfreundliche Terminal-UI
5. Effiziente Docker-Integration

## Technologie-Stack
- Rust (Hauptprogrammiersprache)
- Ratatui (Terminal UI Framework)
- Bitcoin Core RPC
- Docker für Containerisierung
- Tokio für Async-Support

## Anforderungen

### Funktional
1. Node-Status Monitoring
   - Verbindungsstatus
   - Blockchain-Synchronisation
   - Netzwerk-Informationen

2. Wallet-Operationen
   - Balance anzeigen
   - Transaktionshistorie
   - Basis Wallet-Funktionen

3. Benutzeroberfläche
   - Übersichtliches Layout
   - Echtzeit-Updates
   - Tastatursteuerung

### Technisch
1. Zuverlässige RPC-Kommunikation
2. Effizientes Error-Handling
3. Sichere Konfigurationsverwaltung
4. Container-Kompatibilität
5. Performante Aktualisierungen

## Zeitplan/Meilensteine
1. Phase: Grundlegende Infrastruktur
   - [x] Projekt-Setup
   - [x] RPC-Verbindung
   - [x] Basis UI-Framework

2. Phase: Core-Funktionalität
   - [ ] Node-Status Anzeige
   - [ ] Wallet-Integration
   - [ ] Benutzerinteraktion

3. Phase: Erweiterungen
   - [ ] Erweiterte Wallet-Funktionen
   - [ ] Logging/Monitoring
   - [ ] Performance-Optimierung

4. Phase: Finalisierung
   - [ ] Testing
   - [ ] Dokumentation
   - [ ] Release-Vorbereitung