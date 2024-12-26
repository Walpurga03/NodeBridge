# NodeBridge - Bitcoin Node Terminal UI

Eine Terminal-basierte Benutzeroberfläche für Bitcoin Core Nodes, geschrieben in Rust.

## Features

- Echtzeit-Monitoring von Bitcoin Node Metriken
- Automatische Updates alle 5 Sekunden
- Mehrere Ansichten:
  - Übersicht (Node Status)
  - Block Details
  - Mempool Informationen
  - Netzwerk Status
- Farbcodierte Statusanzeigen
- Tastatursteuerung

## Voraussetzungen

- Bitcoin Core Node mit aktivierter RPC-Schnittstelle
- Rust 1.70 oder höher
- Terminal mit UTF-8 Unterstützung

## Installation & Setup

1. Repository klonen:
```bash
git clone https://github.com/Irregular2976/NodeBridge.git
cd NodeBridge
```

2. Umgebungsvariablen konfigurieren:
Erstellen Sie eine `.env` Datei mit folgenden Einträgen:
```env
BTC_RPC_USER=IhrUsername
BTC_RPC_PASSWORD=IhrPasswort
BTC_RPC_HOST=IhrNodeIP
BTC_RPC_PORT=8332
```

3. Kompilieren und Ausführen:
```bash
cargo run
```

## Bedienung

- `1-4`: Zwischen verschiedenen Ansichten wechseln
  - 1: Übersicht
  - 2: Block Details
  - 3: Mempool Status
  - 4: Netzwerk Informationen
- `r`: Manuelles Update der Daten
- `q`: Programm beenden

## Projektstruktur

```
NodeBridge/
├── src/
│   ├── main.rs      # Hauptanwendung
│   ├── rpc/         # Bitcoin RPC Kommunikation
│   └── ui/          # Terminal UI Komponenten
├── Cargo.toml       # Rust Dependencies
├── .env            # Konfiguration (nicht im Git)
└── README.md       # Dokumentation
```

## Entwicklung

Das Projekt verwendet:
- `ratatui` für die Terminal UI
- `bitcoincore-rpc` für Node-Kommunikation
- `tokio` für asynchrone Operationen
- `crossterm` für Terminal-Kontrolle

## Lizenz

MIT License

