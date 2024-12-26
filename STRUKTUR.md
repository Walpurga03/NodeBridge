# Projektstruktur

## Hauptverzeichnis
```
nodebridge/
├── src/              # Quellcode
├── target/           # Build-Artefakte
├── .env              # Konfigurationsdatei für Bitcoin RPC
├── .env.example      # Beispiel-Konfigurationsdatei
├── .gitignore        # Git-Ausschluss Konfiguration
├── Cargo.lock        # Exakte Dependency-Versionen
├── Cargo.toml        # Rust Projekt-Konfiguration
├── frotschritt.md    # Entwicklungsfortschritt
├── MASTERPLAN.md     # Masterplan
├── README.md         # Projekt-Dokumentation
├── STRUKTUR.md       # Diese Datei - Projektstruktur
```

## Source Code Struktur
```
src/
├── main.rs           # Haupteinstiegspunkt der Anwendung
├── rpc/              # Bitcoin RPC Kommunikation
│   └── mod.rs        # RPC Client Implementation
└── ui/               # Terminal User Interface
    ├── mod.rs        # UI Hauptmodul und Zustandsverwaltung
    ├── common.rs     # Gemeinsame Imports und Typen
    ├── components.rs # Wiederverwendbare UI Komponenten
    ├── help.rs       # Hilfe-Overlay Implementation
    ├── render.rs     # Haupt-Rendering Logik
    └── tabs.rs       # Tab-spezifische Ansichten
```

## Detaillierte Beschreibung

### Hauptdateien
- `main.rs`: Initialisiert die Anwendung und startet die UI
- `.env`: Enthält Bitcoin Node Verbindungsdaten (nicht im Git)
- `.env.example`: Zeigt benötigte Umgebungsvariablen

### RPC Modul
- `rpc/mod.rs`:
  - BitcoinRPC Struktur für Node-Kommunikation
  - NodeStatus Struktur für Blockchain-Daten
  - RPC Methoden Implementation
  - Error Handling für RPC Aufrufe

### UI Modul
- `ui/mod.rs`:
  - UI Struktur und Zustandsverwaltung
  - Event Handling (Tastatureingaben)
  - Terminal Setup und Cleanup
  - Update-Zyklus Management

- `ui/common.rs`:
  - Gemeinsame Imports für alle UI Module
  - Gemeinsame Typen und Konstanten
  - Re-Exports häufig genutzter Komponenten

- `ui/components.rs`:
  - Header: Zeigt Bitcoin Core Version
  - Footer: Zeigt Steuerung und Update-Status
  - Tabs: Navigation zwischen Ansichten
  - Wiederverwendbare UI Elemente

- `ui/help.rs`:
  - Hilfe-Overlay Implementation
  - Tastatur-Shortcuts Dokumentation
  - Status-Anzeigen Erklärungen

- `ui/render.rs`:
  - Hauptlogik für UI Rendering
  - Layout Management
  - Tab-Switching Logik
  - Hilfe-Overlay Rendering

- `ui/tabs.rs`:
  - Overview: Allgemeine Node Informationen
  - Block Details: Aktuelle Block Informationen
  - Mempool: Transaktions-Queue Status
  - Network: Verbindungs- und Sync-Status

## Abhängigkeiten
- `ratatui`: Terminal UI Framework
- `crossterm`: Terminal Manipulation
- `bitcoincore-rpc`: Bitcoin Core RPC Client
- `anyhow`: Error Handling
- `chrono`: Zeitstempel Formatierung
- `dotenv`: Umgebungsvariablen

## Konfiguration
Die Anwendung wird über folgende Umgebungsvariablen konfiguriert:
- `RPC_URL`: Bitcoin Node RPC URL
- `RPC_USER`: RPC Benutzername
- `RPC_PASS`: RPC Passwort

## Git-Konfiguration
- `.gitignore`: Definiert Dateien und Verzeichnisse, die nicht versioniert werden:
  ```
  /target          # Kompilierte Binaries und Abhängigkeiten
  **/*.rs.bk      # Rust Backup-Dateien
  .env            # Lokale Konfiguration mit sensiblen Daten
  .idea/          # IntelliJ IDEA Projektdateien
  .vscode/        # Visual Studio Code Einstellungen
  *.pdb          # Windows Debug-Dateien
  ```

## Build-Verzeichnis
```
target/
├── debug/              # Debug Build-Artefakte
│   ├── nodebridge     # Ausführbare Debug-Version
│   ├── deps/          # Kompilierte Abhängigkeiten
│   ├── build/         # Build-Skript Ausgaben
│   ├── examples/      # Kompilierte Beispiele
│   └── incremental/   # Inkrementelle Kompilierung
│
├── release/           # Release Build-Artefakte
│   ├── nodebridge    # Optimierte ausführbare Version
│   ├── deps/         # Optimierte Abhängigkeiten
│   └── build/        # Release Build-Skripte
│
└── doc/              # Generierte Dokumentation
    └── nodebridge/   # API Dokumentation
```