# Projekt-Struktur

## Ordner-Struktur
```
nodebridge/
├── src/
│   ├── main.rs           # Haupteinstiegspunkt
│   ├── rpc/              # Bitcoin RPC Kommunikation
│   │   └── mod.rs        # RPC Client Implementation
│   └── ui/               # User Interface
│       ├── mod.rs        # UI Hauptmodul
│       ├── render.rs     # Rendering-Logik
│       ├── help.rs       # Hilfe-System
│       ├── common.rs     # Gemeinsame UI-Komponenten
│       └── tabs/         # Tab-spezifische UI-Komponenten
│           ├── dashboard.rs
│           ├── block_details.rs
│           ├── mempool.rs
│           ├── network.rs
│           ├── peer_list.rs
│           ├── mining.rs
│           ├── security.rs
│           └── explorer.rs
├── .env                  # Konfigurationsdatei
├── Cargo.toml           # Projekt-Manifest
```

## Komponenten
- **RPC Module**: Bitcoin Core Kommunikation
- **UI Module**: Terminal-basierte Benutzeroberfläche
- **Tab System**: Modulare Ansichten für verschiedene Funktionen
- **Help System**: Kontextbezogene Hilfe
- **Common**: Wiederverwendbare UI-Komponenten
- **Wallet Module**: Verwaltung und Integration von Bitcoin Wallets
- **Export Module**: Funktionen zum Exportieren von Daten in verschiedenen Formaten

## Konfiguration
- `.env`: Bitcoin Core RPC Zugangsdaten
- `Cargo.toml`: Abhängigkeiten und Projekt-Metadaten

## Abhängigkeiten
- **Extern**:
  - ratatui: Terminal UI Framework
  - bitcoincore-rpc: Bitcoin Core RPC Client
  - crossterm: Terminal Control
  - anyhow: Error Handling
  - chrono: Zeitformatierung
  - dotenv: Konfiguration
- **Logging**:
  - log: Logging-Bibliothek
  - env_logger: Logger-Implementierung

- **Intern**:
  - rpc → ui: Node-Daten
  - ui/common → ui/tabs: Gemeinsame Komponenten
  - ui/render → ui/tabs: Tab-Rendering
  - ui/wallet: Verwaltung und Integration von Wallets
  - ui/export: Datenexport-Funktionen